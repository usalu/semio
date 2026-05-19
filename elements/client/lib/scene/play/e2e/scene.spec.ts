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
	expect(text).not.toContain("pseudoZoomFromOrbitDistance is not defined");
}

const SCENE_LOD_TIERS = /^(minimap|overview|compact|normal|detail|micro)$/;

test("scene play loads canvas and fixture", async ({ page }) => {
	const messages = collectSceneConsole(page);
	await page.goto("/");
	await expect(page.locator("canvas")).toBeVisible({ timeout: 120_000 });
	await expect(page.locator("[data-scene-root]")).toBeVisible();
	await page.waitForLoadState("networkidle");
	await page.waitForTimeout(500);
	await expect(page.locator("[data-scene-root]")).toHaveAttribute("data-scene-domain", "architecture");
	await expect.poll(async () => page.locator("[data-e2e-scene-lod]").textContent()).toMatch(SCENE_LOD_TIERS);
	await expect
		.poll(async () => {
			const rootLod = await page.locator("[data-scene-root]").getAttribute("data-scene-lod");
			const label = await page.locator("[data-e2e-scene-lod]").textContent();
			return rootLod === label ? rootLod : null;
		})
		.toMatch(SCENE_LOD_TIERS);
	await expect(page.locator('[data-measure-id="scene-main-lod"]')).toBeVisible({ timeout: 120_000 });
	expectCleanSceneConsole(messages);
});

test("scene play LOD measure pins tier on canvas", async ({ page }) => {
	const messages = collectSceneConsole(page);
	await page.goto("/");
	await page.locator("canvas").first().waitFor({ state: "visible", timeout: 120_000 });
	await expect(page.locator('[data-measure-id="scene-main-lod"]')).toBeVisible({ timeout: 120_000 });
	const lodSelect = page.locator('[data-measure-id="scene-main-lod"]');
	await lodSelect.click();
	await page.getByRole("option", { name: "Minimap" }).click();
	await expect
		.poll(async () => await page.locator("[data-scene-root]").getAttribute("data-scene-lod"), { timeout: 30_000 })
		.toBe("minimap");
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
	const canvas = page.locator("canvas").first();
	await canvas.waitFor({ state: "visible", timeout: 120_000 });
	await page.waitForLoadState("networkidle");
	await page.waitForTimeout(500);
	const before = await page.locator("canvas").count();
	await canvas.click({ position: { x: 320, y: 240 } });
	await page.waitForTimeout(500);
	await expect(page.locator("[data-scene-root]")).toBeVisible();
	await expect(page.locator("canvas")).toHaveCount(before);
	expectCleanSceneConsole(messages);
});

test("scene play serves placeholder mesh as binary glb", async ({ request }) => {
	const response = await request.get("/meshes/placeholder.glb");
	expect(response.ok()).toBe(true);
	expect(response.headers()["content-type"]).toContain("model/gltf-binary");
	const body = await response.body();
	expect(body.subarray(0, 4).toString("ascii")).toBe("glTF");
});
