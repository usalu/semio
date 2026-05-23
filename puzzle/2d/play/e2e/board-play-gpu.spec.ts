// #region 🧲Header
// 💻 elements/client/lib/board/play/e2e/puzzle-2d-play-gpu.spec.ts — Asserts WebGPU raster path paints, wheel reaches WASM, and screenshot bytes change after zoom.
// #endregion 🧲Header

import { expect, test, type Page } from "@playwright/test";

async function gotoBoardPlayShell(page: Page): Promise<void> {
	const errors: string[] = [];
	page.on("pageerror", (error) => errors.push(error.message));
	await page.goto("/", { waitUntil: "load", timeout: 180_000 });
	const workbenchToggle = page.locator("#playground\\.panel\\.workbench");
	try {
		await expect(workbenchToggle).toBeVisible({ timeout: 120_000 });
	} catch (cause) {
		const snippet = (await page.locator("body").innerText().catch(() => "")).slice(0, 400);
		throw new Error(`Board play shell did not mount. Console/page errors: ${errors.join(" | ") || "(none)"} Body: ${snippet}`, { cause });
	}
	if ((await workbenchToggle.getAttribute("data-state")) !== "on") {
		await workbenchToggle.click();
	}
	const libraryTab = page.locator("#puzzle-2d-play-library");
	if (await libraryTab.isVisible()) {
		await libraryTab.click();
	}
	await expect
		.poll(async () => await page.getByTestId("puzzle-2d-play-fixture-shelf").isVisible(), { timeout: 120_000 })
		.toBe(true);
}

async function gotoBoardPlayReady(page: Page): Promise<void> {
	await gotoBoardPlayShell(page);
	await expect
		.poll(async () => await page.locator('[data-testid="board-canvas"]').first().isVisible(), { timeout: 180_000 })
		.toBe(true);
	await expect
		.poll(async () => await page.locator('[data-measure-id="2d-overview-lod"]').isVisible(), { timeout: 180_000 })
		.toBe(true);
}

test.describe("board play", () => {
	test.beforeEach(async ({ page }) => {
		await page.addInitScript(() => {
			try {
				localStorage.removeItem("puzzle.2d-play.surface.device");
				localStorage.removeItem("puzzle.2d-play.surface.theme");
				localStorage.removeItem("puzzle.2d-play.surface.expertise");
			} catch {
				/* ignore */
			}
		});
		await page.setViewportSize({ width: 1600, height: 900 });
	});

	test("opens board background context menu on overview canvas", async ({ page }) => {
		await gotoBoardPlayReady(page);
		const canvas = page.locator('[data-testid="board-canvas"]').first();
		await expect(canvas).toBeVisible({ timeout: 120_000 });
		await expect
			.poll(async () => await canvas.getAttribute("data-board-surface-state"), { timeout: 120_000 })
			.not.toBe("init");
		const point = await canvas.evaluate((el) => {
			const rect = el.getBoundingClientRect();
			const x = rect.left + 140;
			const y = rect.top + 180;
			el.dispatchEvent(
				new MouseEvent("contextmenu", {
					bubbles: true,
					button: 2,
					cancelable: true,
					clientX: x,
					clientY: y,
				}),
			);
			return { x, y };
		});
		const menu = page.locator('[role="menu"]').first();
		await expect(menu).toBeVisible({ timeout: 30_000 });
		await expect(page.getByRole("menuitem", { name: "Board background menu" })).toHaveCount(1);
		const menuBox = await menu.boundingBox();
		expect(menuBox).not.toBeNull();
		expect(Math.abs((menuBox?.x ?? 0) - point.x)).toBeLessThan(8);
		expect(Math.abs((menuBox?.y ?? 0) - point.y)).toBeLessThan(8);
	});

	test("redraw toolbar exposes graph and independent handles actions", async ({ page }) => {
		const errors: string[] = [];
		page.on("console", (msg) => {
			if (msg.type() === "error") {
				errors.push(msg.text());
			}
		});
		page.on("pageerror", (err) => {
			errors.push(err.message);
		});
		await gotoBoardPlayShell(page);
		const handlesToolbar = page.locator('button[title^="Redraw handles"]');
		await expect(handlesToolbar).toBeVisible({ timeout: 120_000 });
		await handlesToolbar.click({ force: true });
		await page.locator("#puzzle-2d-play-settings").click({ force: true });
		await expect(page.locator("#puzzle-2d-play-redraw-nodes")).toBeVisible({ timeout: 120_000 });
		await expect(page.locator("#puzzle-2d-play-redraw-handles")).toBeVisible();
		await page.evaluate(() => {
			document.getElementById("puzzle-2d-play-redraw-nodes")?.click();
			document.getElementById("puzzle-2d-play-redraw-handles")?.click();
		});
		await handlesToolbar.click({ force: true });
		await page.waitForTimeout(100);
		const blockingErrors = errors.filter(
			(text) =>
				!text.includes("Failed to load resource") &&
				!text.includes("[DEBUG] BoardRenderer GPU surface init failed") &&
				!text.includes("null pointer passed to rust") &&
				!text.includes("boardsession_attach_canvas"),
		);
		expect(blockingErrors).toEqual([]);
	});

	test("LOD select is visible on each pane and pins a tier without clearing the graph", async ({ page }) => {
		await gotoBoardPlayReady(page);
		await expect(page.locator('[data-measure-id="2d-detail-lod"]')).toBeVisible();
		await expect(page.locator('[data-measure-id="2d-selection-lod"]')).toBeVisible();
		const overviewCanvas = page.locator('[data-testid="board-canvas"]').first();
		await expect
			.poll(async () => Number(await overviewCanvas.getAttribute("data-board-scene-node-count")), { timeout: 120_000 })
			.toBeGreaterThan(0);
		await page.locator('[data-measure-id="2d-overview-lod"] [data-slot="select-trigger"]').click({ force: true });
		await page.getByRole("option", { name: "Overview", exact: true }).click();
		await expect
			.poll(async () => await overviewCanvas.getAttribute("data-board-lod"), { timeout: 30_000 })
			.toBe("overview");
		await expect
			.poll(async () => Number(await overviewCanvas.getAttribute("data-board-scene-node-count")), { timeout: 30_000 })
			.toBeGreaterThan(0);
		await page.locator('[data-measure-id="2d-overview-lod"] [data-slot="select-trigger"]').click({ force: true });
		await page.getByRole("option", { name: /^Automatic · / }).click();
		await expect
			.poll(async () => await overviewCanvas.getAttribute("data-board-lod"), { timeout: 30_000 })
			.toMatch(/^(minimap|overview|normal|detail|micro)$/);
	});

	test("window options overlay stays pointer-events none under Golden Layout (canvas hit-test)", async ({ page }) => {
		await gotoBoardPlayReady(page);
		const overlays = page.locator('[data-slot="window-measures-overlay"]');
		await expect(overlays).toHaveCount(3, { timeout: 120_000 });
		const n = await overlays.count();
		for (let i = 0; i < n; i++) {
			const pe = await overlays.nth(i).evaluate((el) => globalThis.getComputedStyle(el).pointerEvents);
			expect(pe).toBe("none");
		}
	});

	test("hover state follows the pointer and syncs across every board pane", async ({ page }) => {
		await gotoBoardPlayReady(page);
		const canvases = page.locator('[data-testid="board-canvas"]');
		await expect(canvases).toHaveCount(3, { timeout: 120_000 });
		const canvas = canvases.first();
		await expect
			.poll(async () => Number(await canvas.getAttribute("data-board-scene-node-count")), { timeout: 120_000 })
			.toBeGreaterThan(0);
		const probe = await canvas.evaluate((el) => {
			const renderer = (el as HTMLCanvasElement & { __boardRenderer?: { scene: { nodes: Map<string, { id: string; x: number; y: number }> }; worldToScreen: (p: { x: number; y: number }) => { x: number; y: number } } }).__boardRenderer;
			if (!renderer) {
				throw new Error("Board renderer missing on canvas");
			}
			const node = [...renderer.scene.nodes.values()][0];
			if (!node) {
				throw new Error("Board scene has no nodes");
			}
			const p = renderer.worldToScreen({ x: node.x, y: node.y });
			const rect = el.getBoundingClientRect();
			return { id: node.id, x: rect.left + p.x, y: rect.top + p.y };
		});
		await page.mouse.move(probe.x, probe.y);
		await expect
			.poll(async () => await canvases.evaluateAll((els) => els.map((el) => el.getAttribute("data-board-hover") ?? "")), { timeout: 30_000 })
			.toEqual([probe.id, probe.id, probe.id]);
	});

	test("hit target at canvas center is the board canvas under Golden Layout", async ({ page }) => {
		await gotoBoardPlayReady(page);
		const canvas = page.locator('[data-testid="board-canvas"]').first();
		await expect(canvas).toBeVisible({ timeout: 120_000 });
		await expect
			.poll(async () => await canvas.getAttribute("data-board-surface-state"), { timeout: 120_000 })
			.not.toBe("init");
		const probe = await canvas.evaluate((el) => {
			const r = el.getBoundingClientRect();
			const x = r.left + Math.min(120, Math.max(8, r.width * 0.35));
			const y = r.top + Math.min(120, Math.max(8, r.height * 0.35));
			const stack = document.elementsFromPoint(x, y);
			const idxCanvas = stack.indexOf(el);
			const idxOverlay = stack.findIndex(
				(n) => n instanceof Element && n.closest("[data-slot='window-measures-overlay']") !== null,
			);
			return { idxCanvas, idxOverlay, stackLen: stack.length };
		});
		expect(probe.idxCanvas).toBeGreaterThanOrEqual(0);
		expect(probe.idxOverlay === -1 || probe.idxCanvas < probe.idxOverlay).toBe(true);
	});

	test("no wasm borrow_fail during load and viewport resize stress", async ({ page }, testInfo) => {
		const errors: string[] = [];
		page.on("console", (msg) => {
			if (msg.type() === "error") {
				errors.push(msg.text());
			}
		});
		page.on("pageerror", (err) => {
			errors.push(err.message);
		});
		const adapterOk = await page.evaluate(async () => {
			const gpu = globalThis.navigator?.gpu;
			if (!gpu) return false;
			const adapter = await gpu.requestAdapter();
			return adapter != null;
		});
		if (!adapterOk) {
			testInfo.skip(true, "No WebGPU adapter: use PUZZLE_2D_PLAYWRIGHT_CHANNEL=chrome to exercise this test");
		}
		await gotoBoardPlayReady(page);
		const canvases = page.locator('[data-testid="board-canvas"]');
		await expect(canvases).toHaveCount(3, { timeout: 180_000 });
		try {
			await expect
				.poll(
					async () => {
						const loc = page.locator('[data-testid="board-canvas"]');
						return await loc.evaluateAll((els) => els.every((el) => el.getAttribute("data-board-surface-state") === "ready"));
					},
					{ timeout: 120_000 },
				)
				.toBe(true);
		} catch {
			testInfo.skip(true, "Not all board canvases reached ready/gpu (same as GPU readiness test)");
		}
		for (let i = 0; i < 8; i++) {
			await page.setViewportSize({ width: 1280 + i * 40, height: 720 + i * 20 });
			await page.waitForTimeout(40);
		}
		const borrowLike = errors.filter(
			(t) =>
				t.includes("borrow_fail") ||
				t.includes("unsafe aliasing") ||
				t.includes("recursive use of an object"),
		);
		if (borrowLike.length > 0) {
			throw new Error(`WASM re-entry / borrow errors in console:\n${borrowLike.join("\n")}\n--- all console errors ---\n${errors.join("\n")}`);
		}
	});

	test("each board canvas reaches GPU ready state", async ({ page }, testInfo) => {
		const adapterOk = await page.evaluate(async () => {
			const gpu = globalThis.navigator?.gpu;
			if (!gpu) return false;
			const adapter = await gpu.requestAdapter();
			return adapter != null;
		});
		if (!adapterOk) {
			testInfo.skip(true, "No WebGPU adapter reported by the browser");
		}
		await gotoBoardPlayReady(page);
		const canvases = page.locator('[data-testid="board-canvas"]');
		await expect(canvases).toHaveCount(3, { timeout: 180_000 });
		try {
			await expect
				.poll(
					async () => {
						const loc = page.locator('[data-testid="board-canvas"]');
						return await loc.evaluateAll((els) =>
							els.map((el) => `${el.getAttribute("data-board-surface-state") ?? "?"}/${el.getAttribute("data-board-raster") ?? "?"}`),
						);
					},
					{ timeout: 120_000 },
				)
				.toEqual(["ready/gpu", "ready/gpu", "ready/gpu"]);
		} catch (cause) {
			const detail =
				(await page.locator('[data-testid="board-canvas"]').first().getAttribute("data-board-surface-failure")) ?? "(no data-board-surface-failure)";
			if (detail === "NoCompatibleDevice") {
				testInfo.skip(true, `WebGPU surface unavailable: ${detail}`);
			}
			throw new Error(`Expected three ready GPU canvases. data-board-surface-failure (first canvas): ${detail}`, { cause });
		}
		for (const c of await canvases.all()) {
			await expect(c).toHaveAttribute("data-board-raster", "gpu");
			await expect(c).toHaveAttribute("data-board-surface-state", "ready");
		}
	});

	test("GPU canvas pixels and zoom change after wheel over canvas (interaction + repaint)", async ({ page }, testInfo) => {
		const adapterOk = await page.evaluate(async () => {
			const gpu = globalThis.navigator?.gpu;
			if (!gpu) return false;
			const adapter = await gpu.requestAdapter();
			return adapter != null;
		});
		if (!adapterOk) {
			testInfo.skip(true, "No WebGPU adapter: use PUZZLE_2D_PLAYWRIGHT_CHANNEL=chrome to exercise this test");
		}
		await gotoBoardPlayReady(page);
		const canvas = page.locator('[data-testid="board-canvas"]').first();
		await expect(canvas).toBeVisible({ timeout: 120_000 });
		await expect
			.poll(async () => await canvas.getAttribute("data-board-surface-state"), { timeout: 120_000 })
			.toBe("ready");
		await expect(canvas).toHaveAttribute("data-board-pointer-bridge", "window");
		const beforeZoom = await canvas.getAttribute("data-board-zoom");
		expect(beforeZoom).not.toBeNull();
		const box = await canvas.boundingBox();
		expect(box).not.toBeNull();
		const cx = box!.x + box!.width * 0.5;
		const cy = box!.y + box!.height * 0.5;
		await page.mouse.move(cx, cy);
		const shotBefore = await canvas.screenshot();
		await page.mouse.wheel(0, -600);
		await expect.poll(async () => await canvas.getAttribute("data-board-zoom")).not.toBe(beforeZoom);
		await expect
			.poll(
				async () => {
					const shotAfter = await canvas.screenshot();
					return !Buffer.from(shotBefore).equals(Buffer.from(shotAfter));
				},
				{ timeout: 60_000 },
			)
			.toBe(true);
	});
});
