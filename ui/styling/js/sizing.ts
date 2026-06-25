// #region 🧲Header
/** @emoji 📐 DOM sizing helpers — CSS variable refs and ui-spacing multipliers from {@link STYLING_METRICS.dom}. */
// #endregion 🧲Header

import { STYLING_METRICS } from "./tokens.generated.ts";

//#region 🔑SizeVars
/** @emoji 🔑 Canonical DOM size CSS variable names. */
export const STYLING_SIZE_VAR = {
	uiSpacing: "--ui-spacing",
	spacingSingle: "--spacing-single",
	spacingDouble: "--spacing-double",
	sizeTiny: "--size-tiny",
	sizeSmall: "--size-small",
	sizeWorkbench: "--size-workbench",
	sizeMedium: "--size-medium",
	sizeLarge: "--size-large",
	sizeHuge: "--size-huge",
	sizeMega: "--size-mega",
	sizeGiga: "--size-giga",
	sizeXl: "--size-xl",
	sizeTera: "--size-tera",
	sizePeta: "--size-peta",
	layoutLabel: "--layout-label",
	layoutPopoverMin: "--layout-popover-min",
	layoutPanelMin: "--layout-panel-min",
	layoutPanelRail: "--layout-panel-rail",
	layoutPanelMax: "--layout-panel-max",
	layoutCommandMax: "--layout-command-max",
	layoutEngagementMax: "--layout-engagement-max",
	layoutDeckWidth: "--layout-deck-width",
	layoutDeckHeight: "--layout-deck-height",
	strokeHairline: "--stroke-hairline",
} as const;

/** @emoji 🔑 Returns a `var(--…)` reference for a DOM size token. */
export function sizeVar(key: keyof typeof STYLING_SIZE_VAR): string {
	return `var(${STYLING_SIZE_VAR[key]})`;
}

/** @emoji 🔑 Compact-mode reference root (px) for headless layout math at default 16px root. */
export const STYLING_COMPACT_ROOT_PX = 16;

const COMPACT_UI_SPACING_REM = 0.2;

/** @emoji 📐 Converts a ui-spacing multiplier to rem length. */
export function uiSpacingRem(multiplier: number): string {
	return `${multiplier * COMPACT_UI_SPACING_REM}rem`;
}

/** @emoji 📐 Converts a ui-spacing multiplier to px at the compact reference root. */
export function uiSpacingPx(multiplier: number, rootPx = STYLING_COMPACT_ROOT_PX): number {
	return multiplier * COMPACT_UI_SPACING_REM * rootPx;
}

/** @emoji 📐 DOM layout multipliers (multiples of `--ui-spacing`) from tokens.json. */
export const STYLING_DOM = STYLING_METRICS.dom;

/** @emoji 📐 Resolves a DOM metric key to px at the compact reference root. */
export function domSizePx(key: keyof typeof STYLING_DOM, rootPx = STYLING_COMPACT_ROOT_PX): number {
	return uiSpacingPx(STYLING_DOM[key], rootPx);
}

/** @emoji 📐 Reads a resolved CSS size variable from the document (browser only). */
export function readSizeVarPx(varName: string, element?: Element | null): number {
	if (typeof document === "undefined") {
		return 0;
	}
	const host = element ?? document.documentElement;
	const raw = getComputedStyle(host).getPropertyValue(varName.startsWith("--") ? varName : `--${varName}`);
	return Number.parseFloat(raw) || 0;
}
//#endregion 🔑SizeVars
