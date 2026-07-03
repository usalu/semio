import { describe, expect, it } from "bun:test";
import { resolve } from "node:path";
import {
	clearColorResolveCache,
	currentStylingThemeName,
	resolveColorHex,
	resolveColorRgba,
	resolveSemanticColorHex,
	serializeGraphCanvasThemePaletteJson,
	tokenHex,
	tokenVar,
} from "./index.ts";
import { STYLING_BOARD_THEMES } from "./tokens.generated.ts";
import { puzzle3dLockedExampleMeshBasenames, puzzle3dMeshBasenamesInJson, PLAYGROUND_PLAY_BOOT_INLINE_STYLE, PLAYGROUND_PLAY_BOOT_REVEAL_SCRIPT, PLAYGROUND_PLAY_BOOT_THEME_SCRIPT, playgroundPlayBootHtmlPlugin } from "../vite-elements-assets.ts";
import { PLAYGROUND_LOCKED_EXAMPLE_ENV } from "../../../repo/lib/js/index.ts";

const repoRoot = resolve(import.meta.dir, "../../..");

describe("styling resolve", () => {
	it("tokenVar and tokenHex read generated palette", () => {
		expect(tokenVar("primary")).toBe("var(--color-primary)");
		expect(tokenHex("primary")).toBe("#ff344f");
	});

	it("serializeGraphCanvasThemePaletteJson emits token board theme fields", () => {
		const parsed = JSON.parse(serializeGraphCanvasThemePaletteJson("light")) as {
			rasterClear: number[];
			labelFill: number[];
			gridMinorStroke: number[];
		};
		expect(parsed.rasterClear).toEqual(STYLING_BOARD_THEMES.light.rasterClear);
		expect(parsed.labelFill).toEqual([123, 130, 125, 255]);
		expect(parsed.gridMinorStroke[3]).toBeLessThan(255);
		const dark = JSON.parse(serializeGraphCanvasThemePaletteJson("dark")) as { rasterClear: number[] };
		expect(dark.rasterClear).toEqual(STYLING_BOARD_THEMES.dark.rasterClear);
	});

	it("currentStylingThemeName defaults to light without document", () => {
		expect(currentStylingThemeName()).toBe("light");
	});

	it("resolveColorHex resolves palette var refs headlessly", () => {
		clearColorResolveCache();
		expect(resolveColorHex("var(--color-secondary)", "gray")).toBe("#34d1bf");
		expect(resolveSemanticColorHex("border-element-color", "gray")).toBe("#7b827d");
	});

	it("resolveColorRgba returns byte tuple", () => {
		clearColorResolveCache();
		expect(resolveColorRgba("var(--color-gray)", "gray")).toEqual([123, 130, 125, 255]);
	});

	it("ui.css keeps spacing tokens in @theme inline for production builds", async () => {
		const { readFile } = await import("node:fs/promises");
		const { resolve } = await import("node:path");
		const uiCss = await readFile(resolve(import.meta.dir, "ui.css"), "utf8");
		expect(uiCss).toContain("--ui-spacing: var(--spacing-compact)");
		expect(uiCss).toContain("--spacing-single: calc(1 * var(--ui-spacing))");
		expect(uiCss).toContain("--spacing-double: calc(2 * var(--ui-spacing))");
		expect(uiCss).toContain("--glass-panel-blur: 2.5rem");
		expect(uiCss).toContain("--radius-sm: 0rem");
		expect(uiCss).not.toMatch(/@layer base\s*\{[^}]*:root[^}]*--spacing:\s*var\(--spacing-compact\)/s);
	});

	it("ui.css defines per-level element foreground tokens and panel scoping", async () => {
		const { readFile } = await import("node:fs/promises");
		const { resolve } = await import("node:path");
		const uiCss = await readFile(resolve(import.meta.dir, "ui.css"), "utf8");
		expect(uiCss).toContain("--element-base: var(--color-gray);");
		expect(uiCss).toContain("--element-panel: var(--color-dark-5-9);");
		expect(uiCss).toContain("--border-element-color: var(--element-base);");
		expect(uiCss).toContain('[data-level="panel"] {\n  --border-element-color: var(--element-panel);');
		expect(uiCss).toContain(".dark {\n  --base: var(--color-dark);");
		expect(uiCss).toContain("--element-panel: var(--color-gray-600);");
	});

	it("ui.css left-aligns the footer toolbar and grows it through the remaining width", async () => {
		const { readFile } = await import("node:fs/promises");
		const { resolve } = await import("node:path");
		const uiCss = await readFile(resolve(import.meta.dir, "ui.css"), "utf8");
		expect(uiCss).toContain('[data-slot="toolbar-anchor"] {\n  flex: 1 1 0%;\n  min-width: 0;\n  height: 100%;');
		expect(uiCss).toContain("justify-content: flex-start;");
		expect(uiCss).toContain('[data-slot="toolbar-anchor"] > * {\n  flex: 0 1 auto;\n  width: fit-content;\n  max-width: 100%;\n  min-width: 0;');
		expect(uiCss).toContain('[data-slot="toolbar-anchor"] [role="toolbar"] {\n  max-height: 100%;\n}');
	});
});

describe("puzzle3d mesh build helpers", () => {
	it("collects mesh basenames from fixture JSON", () => {
		const basenames = puzzle3dMeshBasenamesInJson({
			objects: [{ meshUrl: "/mesh/hexagonal-cut-concrete-forest-left.glb" }],
			meta: { kindCatalogs: { objects: [{ meshUrl: "/mesh/capsule_J.glb" }] } },
		});
		expect([...basenames].sort()).toEqual(["capsule_J.glb", "hexagonal-cut-concrete-forest-left.glb"]);
	});

	it("returns only concrete forest glbs when fixture is locked", () => {
		const prev = process.env[PLAYGROUND_LOCKED_EXAMPLE_ENV];
		try {
			process.env[PLAYGROUND_LOCKED_EXAMPLE_ENV] = "concrete-forest";
			const basenames = puzzle3dLockedExampleMeshBasenames(repoRoot);
			expect(basenames?.has("hexagonal-cut-concrete-forest-left.glb")).toBe(true);
			expect(basenames?.has("hexagonal-cut-concrete-forest-right.glb")).toBe(true);
			expect(basenames?.has("capsule_J.glb")).toBe(false);
			expect(basenames?.has("placeholder.glb")).toBe(true);
		} finally {
			if (prev === undefined) delete process.env[PLAYGROUND_LOCKED_EXAMPLE_ENV];
			else process.env[PLAYGROUND_LOCKED_EXAMPLE_ENV] = prev;
		}
	});
});

describe("playground play boot html", () => {
	it("registers the vite index html plugin", () => {
		expect(playgroundPlayBootHtmlPlugin().name).toBe("playground-play-boot-html");
	});

	it("hides the body until the linked stylesheet loads", () => {
		expect(PLAYGROUND_PLAY_BOOT_INLINE_STYLE).toContain("data-semio-styled");
		expect(PLAYGROUND_PLAY_BOOT_THEME_SCRIPT).toContain("prefers-color-scheme");
		expect(PLAYGROUND_PLAY_BOOT_REVEAL_SCRIPT).toContain("semio-play-styles");
	});
});
