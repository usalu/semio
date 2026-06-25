// #region 🧲Header
/** @emoji 🎨 `@semio-tech/ui-styling` centralizes palette CSS, Tailwind entry, and the shared typography preset for ui consumers. */
// #endregion 🧲Header

export { tailwindConfig, tailwindConfig as default } from "../tailwind/tailwind.config.ts";
export {
	STYLING_BOARD_THEMES,
	STYLING_CANVAS_FONTS,
	STYLING_CANVAS_THEMES,
	STYLING_MAP_THEMES,
	STYLING_METRICS,
	STYLING_OPACITIES,
	STYLING_RADII,
	STYLING_STROKES,
	STYLING_TOKENS,
	type StylingThemeName,
	type StylingTokenKey,
} from "./tokens.generated.ts";
export {
	STYLING_SIZE_VAR,
	STYLING_COMPACT_ROOT_PX,
	STYLING_DOM,
	domSizePx,
	readSizeVarPx,
	sizeVar,
	uiSpacingPx,
	uiSpacingRem,
} from "./sizing.ts";
export {
	blendTokenHex,
	clearColorResolveCache,
	hexToThreeColor,
	readableForegroundHex,
	relativeLuminance,
	resolveBackgroundColorHex,
	resolveColorHex,
	resolveColorRgba,
	resolveSemanticColorHex,
	resolveThreeColor,
	semanticVar,
	currentStylingThemeName,
	serializeGraphVelloThemePaletteJson,
	themeColorVar,
	tokenHex,
	tokenVar,
} from "./resolve.ts";
export type {
	IconRenderCamera,
	IconRenderFormat,
	IconRenderShape,
	IconRenderLights,
	IconRenderMaterial,
	IconRenderPort,
	IconRenderRequest,
	IconRenderResult,
} from "./icon-render-port.ts";
