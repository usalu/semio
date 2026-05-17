// #region 🧲Header
// 💻 elements/client/lib/board/play/e2e/board-play-gpu.spec.ts — Asserts WebGPU raster path paints, wheel reaches WASM, and screenshot bytes change after zoom.
// #endregion 🧲Header

import { expect, test } from "@playwright/test";

test.describe("board play", () => {
	test.beforeEach(async ({ page }) => {
		await page.addInitScript(() => {
			try {
				localStorage.removeItem("elements.board-play.surface.device");
				localStorage.removeItem("elements.board-play.surface.theme");
				localStorage.removeItem("elements.board-play.surface.expertise");
			} catch {
				/* ignore */
			}
		});
		await page.setViewportSize({ width: 1600, height: 900 });
	});

	test("opens board background context menu on overview canvas", async ({ page }) => {
		await page.goto("/", { waitUntil: "load", timeout: 180_000 });
		await expect(page.getByTestId("board-play-fixture-shelf")).toBeVisible({ timeout: 120_000 });
		await expect(page.locator('[data-option-id$="-redraw-interactive-zoom"]')).toHaveCount(3);
		await expect(page.locator("#board-overview-redraw-interactive-zoom")).toHaveAttribute("data-state", "off");
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
		await expect(page.getByRole("menuitem", { name: "Board background menu" })).toHaveCount(0);
		const menuBox = await menu.boundingBox();
		expect(menuBox).not.toBeNull();
		expect(Math.abs((menuBox?.x ?? 0) - point.x)).toBeLessThan(8);
		expect(Math.abs((menuBox?.y ?? 0) - point.y)).toBeLessThan(8);
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
			testInfo.skip(true, "No WebGPU adapter: use BOARD_PLAYWRIGHT_CHANNEL=chrome to exercise this test");
		}
		await page.goto("/", { waitUntil: "load", timeout: 180_000 });
		await expect(page.getByTestId("board-play-fixture-shelf")).toBeVisible({ timeout: 120_000 });
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
		await page.goto("/", { waitUntil: "load", timeout: 180_000 });
		const adapterOk = await page.evaluate(async () => {
			const gpu = globalThis.navigator?.gpu;
			if (!gpu) return false;
			const adapter = await gpu.requestAdapter();
			return adapter != null;
		});
		if (!adapterOk) {
			testInfo.skip(true, "No WebGPU adapter reported by the browser");
		}
		await expect(page.getByTestId("board-play-fixture-shelf")).toBeVisible({ timeout: 120_000 });
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
			testInfo.skip(true, "No WebGPU adapter: use BOARD_PLAYWRIGHT_CHANNEL=chrome to exercise this test");
		}
		await page.goto("/", { waitUntil: "load", timeout: 180_000 });
		await expect(page.getByTestId("board-play-fixture-shelf")).toBeVisible({ timeout: 120_000 });
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
