import { expect, test, type Locator, type Page } from "@playwright/test";

function collectPuzzle3dPlayConsole(page: Page): string[] {
	const messages: string[] = [];
	page.on("console", (message) => messages.push(message.text()));
	page.on("pageerror", (error) => messages.push(error.message));
	return messages;
}

function expectCleanPuzzle3dPlayConsole(messages: string[]): void {
	const text = messages.join("\n");
	expect(text).not.toContain("Multiple instances of Three.js");
	expect(text).not.toContain("indirectCount is not defined");
	expect(text).not.toContain("React does not recognize the `asChild` prop");
	expect(text).not.toContain("computeBoundingSphere");
	expect(text).not.toContain("Could not load /meshes/");
	expect(text).not.toContain("An error occurred in the <CanvasImpl> component");
	expect(text).not.toContain("Maximum call stack size exceeded");
	expect(text).not.toContain("updateMatrixWorld");
	expect(text).not.toContain("lodFromCameraDistance is not defined");
	expect(text).not.toContain("must declare items or content");
	expect(text).not.toContain("An error occurred in the <Puzzle3dPlayProductShell> component");
	expect(text).not.toContain("Failed to decode downloaded font");
	expect(text).not.toContain("OTS parsing error");
	expect(text).not.toContain("invalid sfntVersion");
}

const PUZZLE_3D_LOD_NUMERIC = /^\d+(\.\d+)?$/;

async function waitPuzzle3dPlayReady(page: Page): Promise<Locator> {
	await page.goto("/");
	const puzzle3dRoot = page.locator("[data-puzzle3d-root]");
	await expect(puzzle3dRoot).toBeVisible({ timeout: 120_000 });
	await expect(puzzle3dRoot).toHaveAttribute("data-puzzle3d-domain", "architecture", { timeout: 120_000 });
	await expect(puzzle3dRoot.locator("canvas")).toBeVisible({ timeout: 120_000 });
	return puzzle3dRoot;
}

async function waitPuzzle3dPlayHooks(page: Page): Promise<void> {
	await page.waitForFunction(
		() => typeof (window as unknown as { __puzzle3dPlaySelect?: unknown }).__puzzle3dPlaySelect === "function",
		{ timeout: 120_000 },
	);
}

async function ensureDetailsPanelOpen(page: Page): Promise<void> {
	const detailsPanelToggle = page.locator("#playground\\.panel\\.details");
	if ((await detailsPanelToggle.getAttribute("data-state")) !== "on") {
		await detailsPanelToggle.click();
	}
}

async function expectPuzzle3dLodReady(puzzle3dRoot: ReturnType<Page["locator"]>): Promise<void> {
	await expect(puzzle3dRoot).toHaveAttribute("data-puzzle3d-lod", PUZZLE_3D_LOD_NUMERIC, { timeout: 30_000 });
}

test("puzzle 3d play loads canvas and fixture", async ({ page }) => {
	const messages = collectPuzzle3dPlayConsole(page);
	const puzzle3dRoot = await waitPuzzle3dPlayReady(page);
	await expectPuzzle3dLodReady(puzzle3dRoot);
	await expect(page.locator('[data-measure-id="puzzle-3d-main-lod"]')).toBeVisible({ timeout: 120_000 });
	expectCleanPuzzle3dPlayConsole(messages);
});

test("puzzle 3d play LOD measure pins manual lod on canvas", async ({ page }) => {
	const messages = collectPuzzle3dPlayConsole(page);
	await waitPuzzle3dPlayReady(page);
	await ensureDetailsPanelOpen(page);
	await expect(page.locator('[data-measure-id="puzzle-3d-main-auto"]')).toBeVisible({ timeout: 120_000 });
	await page.locator("#puzzle-3d-main-auto").click({ timeout: 30_000 });
	const slider = page.locator('[data-measure-id="puzzle-3d-main-lod"] [role="slider"]').first();
	await slider.scrollIntoViewIfNeeded();
	await expect(slider).toBeVisible({ timeout: 30_000 });
	await slider.focus();
	for (let i = 0; i < 40; i += 1) {
		await page.keyboard.press("ArrowRight");
	}
	await expect
		.poll(async () => await page.locator("[data-puzzle3d-root]").getAttribute("data-puzzle3d-lod"), { timeout: 30_000 })
		.toMatch(PUZZLE_3D_LOD_NUMERIC);
	const pinned = await page.locator("[data-puzzle3d-root]").getAttribute("data-puzzle3d-lod");
	expect(Number(pinned)).toBeGreaterThan(1);
	expectCleanPuzzle3dPlayConsole(messages);
});

test("puzzle 3d play inspector panel is visible", async ({ page }) => {
	const messages = collectPuzzle3dPlayConsole(page);
	await waitPuzzle3dPlayReady(page);
	await ensureDetailsPanelOpen(page);
	await page.locator("#puzzle-3d-play-inspector").click({ timeout: 30_000 });
	await expect(page.getByText("Inspector", { exact: true })).toBeVisible({ timeout: 30_000 });
	expectCleanPuzzle3dPlayConsole(messages);
});

test("puzzle 3d selection hook updates label", async ({ page }) => {
	const messages = collectPuzzle3dPlayConsole(page);
	await waitPuzzle3dPlayReady(page);
	await waitPuzzle3dPlayHooks(page);
	const objectId = "01890804-66f2-4544-98f0-b6f0c0615492";
	const objectLabel = "J · cs_sl1_d0_t_f4_b_c1";
	await page.waitForFunction(
		({ id, label }) => {
			const w = window as unknown as { __puzzle3dPlaySelect?: (objectId: string) => void };
			if (typeof w.__puzzle3dPlaySelect !== "function") return false;
			w.__puzzle3dPlaySelect(id);
			const selected = document.querySelector("[data-e2e-selected]")?.textContent ?? "";
			return selected.includes(label);
		},
		{ id: objectId, label: objectLabel },
		{ timeout: 30_000 },
	);
	expectCleanPuzzle3dPlayConsole(messages);
});

test("puzzle 3d pointer miss clears selection", async ({ page }) => {
	const messages = collectPuzzle3dPlayConsole(page);
	await waitPuzzle3dPlayReady(page);
	await waitPuzzle3dPlayHooks(page);
	const objectId = "01890804-66f2-4544-98f0-b6f0c0615492";
	const objectLabel = "J · cs_sl1_d0_t_f4_b_c1";
	await page.waitForFunction(
		({ id, label }) => {
			const w = window as unknown as { __puzzle3dPlaySelect?: (objectId: string) => void };
			if (typeof w.__puzzle3dPlaySelect !== "function") return false;
			w.__puzzle3dPlaySelect(id);
			const selected = document.querySelector("[data-e2e-selected]")?.textContent ?? "";
			return selected.includes(label);
		},
		{ id: objectId, label: objectLabel },
		{ timeout: 30_000 },
	);
	await page.evaluate(() => {
		const w = window as unknown as { __puzzle3dPlayPointerMiss?: () => void };
		if (typeof w.__puzzle3dPlayPointerMiss !== "function") {
			throw new Error("missing __puzzle3dPlayPointerMiss");
		}
		w.__puzzle3dPlayPointerMiss();
	});
	await expect(page.locator("[data-e2e-selected]")).toHaveText("none", { timeout: 15_000 });
	expectCleanPuzzle3dPlayConsole(messages);
});

test("puzzle 3d activate hook shows relocate controls without recursion", async ({ page }) => {
	const messages = collectPuzzle3dPlayConsole(page);
	await waitPuzzle3dPlayReady(page);
	await waitPuzzle3dPlayHooks(page);
	const objectId = "01890804-66f2-4544-98f0-b6f0c0615492";
	const objectLabel = "J · cs_sl1_d0_t_f4_b_c1";
	await page.waitForFunction(
		({ id, label }) => {
			const w = window as unknown as { __puzzle3dPlayActivate?: (objectId: string) => void };
			if (typeof w.__puzzle3dPlayActivate !== "function") return false;
			w.__puzzle3dPlayActivate(id);
			const selected = document.querySelector("[data-e2e-selected]")?.textContent ?? "";
			return selected.includes(label);
		},
		{ id: objectId, label: objectLabel },
		{ timeout: 30_000 },
	);
	await expect(page.locator("canvas")).toBeVisible();
	await page.waitForTimeout(250);
	expectCleanPuzzle3dPlayConsole(messages);
});

test("puzzle 3d does not return to loading meshes after initial load", async ({ page }) => {
	const messages = collectPuzzle3dPlayConsole(page);
	await waitPuzzle3dPlayReady(page);
	await expect(page.getByText("Loading meshes…")).toHaveCount(0, { timeout: 120_000 });
	await page.waitForTimeout(2000);
	await expect(page.getByText("Loading meshes…")).toHaveCount(0);
	expectCleanPuzzle3dPlayConsole(messages);
});

test("puzzle 3d click keeps chunked meshes mounted", async ({ page }) => {
	const messages = collectPuzzle3dPlayConsole(page);
	const puzzle3dRoot = await waitPuzzle3dPlayReady(page);
	const canvas = puzzle3dRoot.locator("canvas").first();
	await expect.poll(async () => page.locator("canvas").count()).toBeGreaterThan(0);
	const before = await page.locator("canvas").count();
	await canvas.click({ position: { x: 320, y: 240 } });
	await expect(puzzle3dRoot).toBeVisible();
	await expect.poll(async () => page.locator("canvas").count()).toBe(before);
	expectCleanPuzzle3dPlayConsole(messages);
});

test("puzzle 3d camera motion changes canvas pixels", async ({ page }) => {
	const messages = collectPuzzle3dPlayConsole(page);
	const puzzle3dRoot = await waitPuzzle3dPlayReady(page);
	const canvas = puzzle3dRoot.locator("canvas").first();
	const box = await canvas.boundingBox();
	expect(box).not.toBeNull();
	const cx = box!.x + box!.width * 0.5;
	const cy = box!.y + box!.height * 0.5;
	await page.mouse.move(cx, cy);
	const beforeZoom = await canvas.screenshot();
	for (let i = 0; i < 6; i += 1) {
		await page.mouse.wheel(0, -2000);
	}
	await expect
		.poll(
			async () => {
				const afterZoom = await canvas.screenshot();
				return !Buffer.from(beforeZoom).equals(Buffer.from(afterZoom));
			},
			{ timeout: 30_000 },
		)
		.toBe(true);
	const beforePan = await canvas.screenshot();
	await page.mouse.move(cx, cy);
	await page.mouse.down({ button: "middle" });
	await page.mouse.move(cx + 140, cy + 80, { steps: 12 });
	await page.mouse.up({ button: "middle" });
	await expect
		.poll(
			async () => {
				const afterPan = await canvas.screenshot();
				return !Buffer.from(beforePan).equals(Buffer.from(afterPan));
			},
			{ timeout: 30_000 },
		)
		.toBe(true);
	expectCleanPuzzle3dPlayConsole(messages);
});

test("puzzle 3d play serves placeholder mesh as binary glb", async ({ request }) => {
	const response = await request.get("/meshes/placeholder.glb");
	expect(response.ok()).toBe(true);
	expect(response.headers()["content-type"]).toContain("model/gltf-binary");
	const body = await response.body();
	expect(body.subarray(0, 4).toString("ascii")).toBe("glTF");
});
