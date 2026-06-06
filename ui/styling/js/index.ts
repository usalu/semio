// #region 🧲Header
/** @emoji 🎨 `@ui/styling` centralizes palette CSS, Tailwind entry, and the shared typography preset for ui consumers. */
// #endregion 🧲Header

export { tailwindConfig, tailwindConfig as default } from "../tailwind/tailwind.config.ts";
export { STYLING_TOKENS, type StylingTokenKey } from "./tokens.generated.ts";
export {
	blendTokenHex,
	clearColorResolveCache,
	hexToThreeColor,
	resolveBackgroundColorHex,
	resolveColorHex,
	resolveColorRgba,
	resolveSemanticColorHex,
	resolveThreeColor,
	semanticVar,
	themeColorVar,
	tokenHex,
	tokenVar,
} from "./resolve.ts";
