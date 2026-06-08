import { describe, expect, it } from "bun:test";
import {
	clearColorResolveCache,
	currentStylingThemeName,
	resolveColorHex,
	resolveColorRgba,
	resolveSemanticColorHex,
	serializeGraphVelloThemePaletteJson,
	tokenHex,
	tokenVar,
} from "./resolve.ts";
import { STYLING_BOARD_THEMES } from "./tokens.generated.ts";

describe("styling resolve", () => {
	it("tokenVar and tokenHex read generated palette", () => {
		expect(tokenVar("primary")).toBe("var(--color-primary)");
		expect(tokenHex("primary")).toBe("#ff344f");
	});

	it("serializeGraphVelloThemePaletteJson emits token board theme fields", () => {
		const parsed = JSON.parse(serializeGraphVelloThemePaletteJson("light")) as {
			rasterClear: number[];
			labelFill: number[];
			gridMinorStroke: number[];
		};
		expect(parsed.rasterClear).toEqual(STYLING_BOARD_THEMES.light.rasterClear);
		expect(parsed.labelFill).toEqual([123, 130, 125, 255]);
		expect(parsed.gridMinorStroke[3]).toBeLessThan(255);
		const dark = JSON.parse(serializeGraphVelloThemePaletteJson("dark")) as { rasterClear: number[] };
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
});
