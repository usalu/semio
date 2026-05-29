import { expect, test, type Page } from "@playwright/test";

function collectSceneConsole(page: Page): string[] {
	const messages: string[] = [];
	page.on("console", (message) => messages.push(message.text()));
	page.on("pageerror", (error) => messages.push(error.message));
	return messages;
}

function expectCleanSceneConsole(messages: string[]): void {
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
	expect(text).not.toContain("An error occurred in the <ScenePlayProductShell> component");
}

const SCENE_LOD_NUMERIC = /^\d+(\.\d+)?$/;

async function expectSceneLodReady(page: Page): Promise<void> {
	await expect
		.poll(async () => page.locator("[data-scene-root]").getAttribute("data-scene-lod"), { timeout: 120_000 })
		.toMatch(SCENE_LOD_NUMERIC);
	await expect(page.locator("[data-scene-root]")).toHaveAttribute("data-scene-lod", SCENE_LOD_NUMERIC);
}

test("scene play loads canvas and fixture", async ({ page }) => {
	const messages = collectSceneConsole(page);
	await page.goto("/");
	await expect(page.locator("canvas")).toBeVisible({ timeout: 120_000 });
	await expect(page.locator("[data-scene-root]")).toBeVisible();
	await page.waitForLoadState("networkidle");
	await page.waitForTimeout(500);
	await expect(page.locator("[data-scene-root]")).toHaveAttribute("data-scene-domain", "architecture");
	await expectSceneLodReady(page);
	await expect(page.locator('[data-measure-id="scene-main-lod"]')).toBeVisible({ timeout: 120_000 });
	expectCleanSceneConsole(messages);
});

test("scene play LOD measure pins manual lod on canvas", async ({ page }) => {
	const messages = collectSceneConsole(page);
	await page.goto("/");
	await page.locator("canvas").first().waitFor({ state: "visible", timeout: 120_000 });
	await expect(page.locator('[data-measure-id="scene-main-auto"]')).toBeVisible({ timeout: 120_000 });
	const detailsPanelToggle = page.locator("#playground\\.panel\\.details");
	if ((await detailsPanelToggle.getAttribute("data-state")) === "on") {
		await detailsPanelToggle.click();
	}
	await page.locator("#scene-main-auto").click({ timeout: 30_000 });
	const slider = page.locator('[data-measure-id="scene-main-lod"] [role="slider"]');
	await slider.waitFor({ state: "visible", timeout: 30_000 });
	await slider.focus();
	for (let i = 0; i < 40; i += 1) {
		await page.keyboard.press("ArrowRight");
	}
	await expect
		.poll(async () => await page.locator("[data-scene-root]").getAttribute("data-scene-lod"), { timeout: 30_000 })
		.toMatch(SCENE_LOD_NUMERIC);
	const pinned = await page.locator("[data-scene-root]").getAttribute("data-scene-lod");
	expect(Number(pinned)).toBeGreaterThan(1);
	expectCleanSceneConsole(messages);
});

test("scene play inspector panel is visible", async ({ page }) => {
	const messages = collectSceneConsole(page);
	await page.goto("/");
	await page.locator("canvas").first().waitFor({ state: "visible", timeout: 120_000 });
	const detailsPanelToggle = page.locator("#playground\\.panel\\.details");
	if ((await detailsPanelToggle.getAttribute("data-state")) !== "on") {
		await detailsPanelToggle.click();
	}
	await page.locator("#scene-play-inspector").click({ timeout: 30_000 });
	await expect(page.getByText("Inspector", { exact: true })).toBeVisible({ timeout: 30_000 });
	expectCleanSceneConsole(messages);
});

test("scene selection hook updates label", async ({ page }) => {
	const messages = collectSceneConsole(page);
	await page.goto("/");
	await page.locator("canvas").first().waitFor({ state: "visible", timeout: 120_000 });
	await page.waitForLoadState("networkidle");
	const objectId = "01890804-66f2-4544-98f0-b6f0c0615492";
	await page.waitForFunction(
		(id) => {
			const w = window as unknown as { __scenePlaySelect?: (objectId: string) => void };
			if (typeof w.__scenePlaySelect !== "function") return false;
			w.__scenePlaySelect(id);
			const label = document.querySelector("[data-e2e-selected]")?.textContent ?? "";
			return label.includes(id.slice(0, 8));
		},
		objectId,
		{ timeout: 30_000 },
	);
	expectCleanSceneConsole(messages);
});

test("scene background click clears selection", async ({ page }) => {
	const messages = collectSceneConsole(page);
	await page.goto("/");
	const canvas = page.locator("canvas").first();
	await canvas.waitFor({ state: "visible", timeout: 120_000 });
	await page.waitForLoadState("networkidle");
	const objectId = "01890804-66f2-4544-98f0-b6f0c0615492";
	await page.waitForFunction(
		(id) => {
			const w = window as unknown as { __scenePlaySelect?: (objectId: string) => void };
			if (typeof w.__scenePlaySelect !== "function") return false;
			w.__scenePlaySelect(id);
			const label = document.querySelector("[data-e2e-selected]")?.textContent ?? "";
			return label.includes(id.slice(0, 8));
		},
		objectId,
		{ timeout: 30_000 },
	);
	const box = await canvas.boundingBox();
	if (!box) {
		throw new Error("scene canvas missing bounding box");
	}
	await canvas.click({ position: { x: 8, y: 8 } });
	await expect(page.locator("[data-e2e-selected]")).toHaveText("none", { timeout: 15_000 });
	expectCleanSceneConsole(messages);
});

test("scene activate hook shows relocate controls without recursion", async ({ page }) => {
	const messages = collectSceneConsole(page);
	await page.goto("/");
	await page.locator("canvas").first().waitFor({ state: "visible", timeout: 120_000 });
	await page.waitForLoadState("networkidle");
	const objectId = "01890804-66f2-4544-98f0-b6f0c0615492";
	await page.waitForFunction(
		(id) => {
			const w = window as unknown as { __scenePlayActivate?: (objectId: string) => void };
			if (typeof w.__scenePlayActivate !== "function") return false;
			w.__scenePlayActivate(id);
			const label = document.querySelector("[data-e2e-selected]")?.textContent ?? "";
			return label.includes(id.slice(0, 8));
		},
		objectId,
		{ timeout: 30_000 },
	);
	await expect(page.locator("canvas")).toBeVisible();
	await page.waitForTimeout(250);
	expectCleanSceneConsole(messages);
});

test("scene does not return to loading meshes after initial load", async ({ page }) => {
	const messages = collectSceneConsole(page);
	await page.goto("/");
	await page.locator("canvas").first().waitFor({ state: "visible", timeout: 120_000 });
	await page.waitForLoadState("networkidle");
	await expect(page.getByText("Loading meshes…")).toHaveCount(0, { timeout: 120_000 });
	await page.waitForTimeout(2000);
	await expect(page.getByText("Loading meshes…")).toHaveCount(0);
	expectCleanSceneConsole(messages);
});

test("scene click keeps chunked meshes mounted", async ({ page }) => {
	const messages = collectSceneConsole(page);
	await page.goto("/");
	await page.locator("canvas").first().waitFor({ state: "visible", timeout: 120_000 });
	await page.waitForLoadState("networkidle");
	await expect.poll(async () => page.locator("canvas").count()).toBeGreaterThan(0);
	const before = await page.locator("canvas").count();
	await page.locator("canvas").first().click({ position: { x: 320, y: 240 } });
	await expect(page.locator("[data-scene-root]")).toBeVisible();
	await expect.poll(async () => page.locator("canvas").count()).toBe(before);
	expectCleanSceneConsole(messages);
});

test("scene camera motion changes canvas pixels", async ({ page }) => {
	const messages = collectSceneConsole(page);
	await page.goto("/");
	const canvas = page.locator("canvas").first();
	await canvas.waitFor({ state: "visible", timeout: 120_000 });
	await page.waitForLoadState("networkidle");
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
	expectCleanSceneConsole(messages);
});

test("scene play serves placeholder mesh as binary glb", async ({ request }) => {
	const response = await request.get("/meshes/placeholder.glb");
	expect(response.ok()).toBe(true);
	expect(response.headers()["content-type"]).toContain("model/gltf-binary");
	const body = await response.body();
	expect(body.subarray(0, 4).toString("ascii")).toBe("glTF");
});
