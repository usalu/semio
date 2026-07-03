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
import { STYLING_BOARD_THEMES, STYLING_METRICS, STYLING_TOKENS, type StylingTokenKey } from "./tokens.generated.ts";

//#region 🔖sizing
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
//#endregion 🔖sizing

//#region 🔖resolve
//#region 🔑TokenRefs
/** @emoji 🔑 Builds a primitive palette CSS variable reference (`var(--color-<key>)`). */
export function tokenVar(key: StylingTokenKey | string): string {
	return `var(--color-${key.replaceAll("_", "-")})`;
}

/** @emoji 🔑 Builds a semantic UI CSS variable reference (`var(--<name>)`). */
export function semanticVar(name: string): string {
	const trimmed = name.startsWith("--") ? name.slice(2) : name;
	return `var(--${trimmed})`;
}

/** @emoji 🔑 Builds a Tailwind `@theme inline` color alias (`var(--color-<name>)`). */
export function themeColorVar(name: string): string {
	return `var(--color-${name.replaceAll("_", "-")})`;
}

/** @emoji 🔑 Returns the canonical hex for a palette token key (headless-safe). */
export function tokenHex(key: StylingTokenKey | string): string {
	return STYLING_TOKENS[key as StylingTokenKey] ?? STYLING_TOKENS.gray;
}
//#endregion 🔑TokenRefs

//#region 🎨Resolve
const _resolveCache = new Map<string, string>();
const _readableForegroundCache = new Map<string, string>();

/** @emoji 🔄 Clears the color resolve cache (theme switches / tests). */
export function clearColorResolveCache(): void {
	_resolveCache.clear();
	_readableForegroundCache.clear();
}

function normalizeHex(hex: string): string {
	const raw = hex.trim();
	if (/^#[0-9a-f]{3}$/iu.test(raw)) {
		const r = raw[1]!;
		const g = raw[2]!;
		const b = raw[3]!;
		return `#${r}${r}${g}${g}${b}${b}`.toLowerCase();
	}
	if (/^#[0-9a-f]{6}$/iu.test(raw)) {
		return raw.toLowerCase();
	}
	if (/^#[0-9a-f]{8}$/iu.test(raw)) {
		return raw.toLowerCase();
	}
	return raw;
}

function isHexColor(value: string): boolean {
	return /^#[0-9a-f]{3,8}$/iu.test(value.trim());
}

function hexChannel(hex: string, start: number): number {
	return Number.parseInt(hex.slice(start, start + 2), 16);
}

function rgbToHex(r: number, g: number, b: number): string {
	const clamp = (n: number) => Math.min(255, Math.max(0, Math.round(n)));
	return `#${clamp(r).toString(16).padStart(2, "0")}${clamp(g).toString(16).padStart(2, "0")}${clamp(b).toString(16).padStart(2, "0")}`;
}

/** @emoji 🎨 Linear sRGB blend between two palette token keys (headless color-mix approximation). */
export function blendTokenHex(keyA: StylingTokenKey | string, keyB: StylingTokenKey | string, ratioA: number): string {
	const a = normalizeHex(tokenHex(keyA));
	const b = normalizeHex(tokenHex(keyB));
	const t = Math.min(1, Math.max(0, ratioA));
	const r = hexChannel(a, 1) * t + hexChannel(b, 1) * (1 - t);
	const g = hexChannel(a, 3) * t + hexChannel(b, 3) * (1 - t);
	const bl = hexChannel(a, 5) * t + hexChannel(b, 5) * (1 - t);
	return rgbToHex(r, g, bl);
}

function headlessTokenFromVarRef(ref: string): string | undefined {
	const m = ref.match(/^var\(\s*(--color-[a-z0-9-]+)\s*\)$/iu);
	if (!m) {
		return undefined;
	}
	const key = m[1]!.slice("--color-".length);
	return STYLING_TOKENS[key as StylingTokenKey];
}

const SEMANTIC_HEADLESS_FALLBACK: Partial<Record<string, StylingTokenKey>> = {
	"border-element-color": "gray",
	"border-normal-color": "gray",
	"border-emphasized-color": "dark",
	"hover-interactive-fill": "gray",
	foreground: "dark",
};

function headlessSemanticFromVarRef(ref: string): string | undefined {
	const m = ref.match(/^var\(\s*(--[a-z0-9-]+)\s*\)$/iu);
	if (!m) {
		return undefined;
	}
	const key = SEMANTIC_HEADLESS_FALLBACK[m[1]!.slice(2)];
	return key ? tokenHex(key) : undefined;
}

function cssProbeAvailable(): boolean {
	if (typeof document === "undefined" || import.meta.env?.VITEST) {
		return false;
	}
	try {
		return document.createElement("canvas").getContext("2d") != null;
	} catch {
		return false;
	}
}

function probeCssComputed(property: "color" | "backgroundColor", value: string): string {
	if (typeof document === "undefined") {
		return "";
	}
	const el = document.createElement("span");
	const key = property === "color" ? "color" : "background-color";
	el.setAttribute("style", `${key}:${value};position:absolute;left:0;top:0;visibility:hidden;pointer-events:none`);
	if (document.documentElement.classList.contains("dark")) {
		el.classList.add("dark");
	}
	document.documentElement.appendChild(el);
	const out = getComputedStyle(el)[property];
	el.remove();
	return out;
}

function cssPaintToHex(css: string, fallback: string): string {
	if (!css || css === "rgba(0, 0, 0, 0)") {
		return fallback;
	}
	if (isHexColor(css)) {
		return normalizeHex(css);
	}
	if (/^rgba?\(/iu.test(css)) {
		const m = css.match(/rgba?\(\s*([\d.]+)\s*,\s*([\d.]+)\s*,\s*([\d.]+)/u);
		if (m) {
			return rgbToHex(Number(m[1]), Number(m[2]), Number(m[3]));
		}
	}
	if (typeof document === "undefined") {
		return fallback;
	}
	const canvas = document.createElement("canvas");
	let ctx: CanvasRenderingContext2D | null = null;
	try {
		ctx = canvas.getContext("2d");
	} catch {
		return fallback;
	}
	if (!ctx) {
		return fallback;
	}
	ctx.fillStyle = "#000000";
	ctx.fillStyle = css;
	const converted = ctx.fillStyle;
	if (typeof converted === "string" && isHexColor(converted)) {
		return normalizeHex(converted);
	}
	if (typeof converted === "string" && /^rgba?\(/iu.test(converted)) {
		return cssPaintToHex(converted, fallback);
	}
	return fallback;
}

function resolvePaintExpressionHex(trimmed: string, fallback: string, onResolved?: (hex: string) => void): string {
	if (isHexColor(trimmed)) {
		const hex = normalizeHex(trimmed);
		onResolved?.(hex);
		return hex;
	}
	if (cssProbeAvailable()) {
		const raw = probeCssComputed("backgroundColor", trimmed);
		if (raw && raw !== "rgba(0, 0, 0, 0)") {
			const hex = cssPaintToHex(raw, "");
			if (isHexColor(hex)) {
				onResolved?.(hex);
				return hex;
			}
		}
	}
	const palette = headlessTokenFromVarRef(trimmed) ?? headlessSemanticFromVarRef(trimmed);
	if (palette) {
		onResolved?.(palette);
		return palette;
	}
	onResolved?.(fallback);
	return fallback;
}

/** @emoji 🎨 Resolves a CSS color expression or hex literal to `#rrggbb`, using palette fallback in headless mode. */
export function resolveColorHex(ref: string, fallbackKey: StylingTokenKey | string = "gray"): string {
	const cacheKey = `${ref}|${fallbackKey}`;
	const cached = _resolveCache.get(cacheKey);
	if (cached !== undefined) {
		return cached;
	}
	const fallback = tokenHex(fallbackKey);
	return resolvePaintExpressionHex(ref.trim(), fallback, (hex) => {
		_resolveCache.set(cacheKey, hex);
	});
}

/** @emoji 🎨 Resolves a semantic CSS custom property (e.g. `--foreground`) to `#rrggbb`. */
export function resolveSemanticColorHex(cssVar: string, fallbackKey: StylingTokenKey | string = "gray"): string {
	const name = cssVar.startsWith("--") ? cssVar : `--${cssVar}`;
	return resolveBackgroundColorHex(`var(${name})`, fallbackKey);
}

/** @emoji 🎨 Resolves a CSS background-color expression to `#rrggbb`. */
export function resolveBackgroundColorHex(ref: string, fallbackKey: StylingTokenKey | string = "gray"): string {
	const cacheKey = `bg|${ref}|${fallbackKey}`;
	const cached = _resolveCache.get(cacheKey);
	if (cached !== undefined) {
		return cached;
	}
	const fallback = tokenHex(fallbackKey);
	return resolvePaintExpressionHex(ref.trim(), fallback, (hex) => {
		_resolveCache.set(cacheKey, hex);
	});
}

/** @emoji 🎨 Resolves a CSS color expression to RGBA8888 for canvas WASM theme payloads. */
export function resolveColorRgba(
	ref: string,
	fallbackKey: StylingTokenKey | string = "gray",
	alpha = 255,
): [number, number, number, number] {
	const hex = normalizeHex(resolveColorHex(ref, fallbackKey));
	const a = hex.length === 9 ? Number.parseInt(hex.slice(7, 9), 16) : alpha;
	return [hexChannel(hex, 1), hexChannel(hex, 3), hexChannel(hex, 5), a];
}

/** @emoji 🎨 Converts `#rrggbb` to a Three.js-friendly hex number (`0xrrggbb`). */
export function hexToThreeColor(hex: string): number {
	const norm = normalizeHex(hex);
	return Number.parseInt(norm.slice(1, 7), 16);
}

/** @emoji 🎨 Resolves a CSS color expression to a Three.js-friendly hex number. */
export function resolveThreeColor(ref: string, fallbackKey: StylingTokenKey | string = "gray"): number {
	return hexToThreeColor(resolveColorHex(ref, fallbackKey));
}

function srgbChannelToLinear(channel: number): number {
	const s = channel / 255;
	return s <= 0.04045 ? s / 12.92 : ((s + 0.055) / 1.055) ** 2.4;
}

/** @emoji 🌓 WCAG relative luminance for a resolved `#rrggbb` color (0 = black, 1 = white). */
export function relativeLuminance(hex: string): number {
	const norm = normalizeHex(hex);
	const r = srgbChannelToLinear(hexChannel(norm, 1));
	const g = srgbChannelToLinear(hexChannel(norm, 3));
	const b = srgbChannelToLinear(hexChannel(norm, 5));
	return 0.2126 * r + 0.7152 * g + 0.0722 * b;
}

/** @emoji 🏷️ Picks a readable palette foreground hex for text on the given background color expression. */
export function readableForegroundHex(
	backgroundRef: string,
	lightKey: StylingTokenKey | string = "light",
	darkKey: StylingTokenKey | string = "dark",
): string {
	const cacheKey = `${backgroundRef}|${lightKey}|${darkKey}`;
	const cached = _readableForegroundCache.get(cacheKey);
	if (cached !== undefined) {
		return cached;
	}
	const bgHex = resolveBackgroundColorHex(backgroundRef, "gray");
	const result = relativeLuminance(bgHex) > 0.5 ? tokenHex(darkKey) : tokenHex(lightKey);
	_readableForegroundCache.set(cacheKey, result);
	return result;
}

/** @emoji 🌓 Resolves the active styling theme name from the document root class list. */
export function currentStylingThemeName(): StylingThemeName {
	if (typeof document !== "undefined" && document.documentElement.classList.contains("dark")) {
		return "dark";
	}
	return "light";
}

/** @emoji 🎨 Serializes token board theme paints for DAG/flow canvas WASM (`CanvasThemePalette` JSON). */
export function serializeGraphCanvasThemePaletteJson(themeName: StylingThemeName = currentStylingThemeName()): string {
	return JSON.stringify(STYLING_BOARD_THEMES[themeName]);
}

/** @emoji 🎨 WASM session surface that accepts serialized canvas theme palette JSON. */
export interface CanvasThemeSession {
	setCanvasThemeJson(json: string): void;
}

/** @emoji 🌓 Pushes the active document theme palette into a canvas WASM session. */
export function syncSessionCanvasTheme(session: CanvasThemeSession | null | undefined): void {
	if (!session) return;
	try {
		clearColorResolveCache();
		session.setCanvasThemeJson(serializeGraphCanvasThemePaletteJson());
	} catch {
		/* theme tokens not ready */
	}
}
//#endregion 🎨Resolve

//#region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("styling resolve", () => {
		it("tokenVar and tokenHex read generated palette", () => {
			expect(tokenVar("primary")).toBe("var(--color-primary)");
			expect(tokenHex("primary")).toBe("#ff344f");
		});

		it("resolveColorHex resolves palette var refs headlessly", () => {
			clearColorResolveCache();
			expect(resolveColorHex("var(--color-secondary)", "gray")).toBe("#34d1bf");
		});

		it("resolveColorHex passes through hex literals", () => {
			clearColorResolveCache();
			expect(resolveColorHex("#abc", "gray")).toBe("#aabbcc");
		});

		it("blendTokenHex mixes two palette keys", () => {
			const mixed = blendTokenHex("primary", "light", 0.28);
			expect(mixed).toMatch(/^#[0-9a-f]{6}$/u);
		});

		it("resolveColorRgba returns byte tuple", () => {
			clearColorResolveCache();
			expect(resolveColorRgba("var(--color-gray)", "gray")).toEqual([123, 130, 125, 255]);
		});

		it("relativeLuminance orders light above dark palette tokens", () => {
			expect(relativeLuminance(tokenHex("light"))).toBeGreaterThan(relativeLuminance(tokenHex("dark")));
		});

		it("readableForegroundHex picks light text on dark fills and dark text on light fills", () => {
			clearColorResolveCache();
			expect(readableForegroundHex("var(--color-dark)")).toBe(tokenHex("light"));
			expect(readableForegroundHex("var(--color-light)")).toBe(tokenHex("dark"));
		});

		it("resolveColorHex resolves semantic element vars to gray not foreground", () => {
			clearColorResolveCache();
			expect(resolveColorHex("var(--color-element)", "gray")).toBe("#7b827d");
			expect(resolveSemanticColorHex("border-element-color", "gray")).toBe("#7b827d");
			expect(resolveColorHex("var(--color-element)", "gray")).not.toBe(tokenHex("dark"));
		});

		it("serializeGraphCanvasThemePaletteJson emits token board theme fields", () => {
			const parsed = JSON.parse(serializeGraphCanvasThemePaletteJson("light")) as {
				rasterClear: number[];
				nodeFill: number[];
				nodeStroke: number[];
				nodeStrokeHovered: number[];
				nodeStrokeSelected: number[];
				edgeStroke: number[];
				handleStroke: number[];
				handleStrokeHovered: number[];
				handleFill: number[];
				labelFill: number[];
				labelFillHovered: number[];
				labelHalo: number[];
				gridMinorStroke: number[];
			};
			expect(parsed.rasterClear).toEqual(STYLING_BOARD_THEMES.light.rasterClear);
			expect(parsed.nodeFill).toHaveLength(4);
			expect(parsed.labelFill).toEqual([123, 130, 125, 255]);
			expect(parsed.edgeStroke).toEqual([123, 130, 125, 255]);
			expect(parsed.handleStroke).toEqual([123, 130, 125, 255]);
			expect(parsed.handleStrokeHovered).toEqual(parsed.handleStroke);
			expect(parsed.nodeStrokeSelected).toEqual(STYLING_BOARD_THEMES.light.nodeStrokeSelected);
			expect(parsed.handleFill[3]).toBe(0);
			expect(parsed.gridMinorStroke[3]).toBeLessThan(255);
			const dark = JSON.parse(serializeGraphCanvasThemePaletteJson("dark")) as { rasterClear: number[] };
			expect(dark.rasterClear).toEqual(STYLING_BOARD_THEMES.dark.rasterClear);
			expect(dark.rasterClear).not.toEqual(parsed.rasterClear);
		});

		it("syncSessionCanvasTheme pushes serialized palette into a session", () => {
			const calls: string[] = [];
			syncSessionCanvasTheme({
				setCanvasThemeJson(json: string) {
					calls.push(json);
				},
			});
			expect(calls.length).toBe(1);
			const parsed = JSON.parse(calls[0]!) as { rasterClear: number[] };
			expect(parsed.rasterClear).toEqual(STYLING_BOARD_THEMES.light.rasterClear);
		});
	});
}
//#endregion 🧪Tests
//#endregion 🔖resolve

//#region 🔖icon-render-port
//#region 🔖IconRenderPort
export type IconRenderFormat = "svg" | "png";

export type IconRenderShape = "rectangle" | "ellipse";

export interface IconRenderCamera {
	readonly position: readonly [number, number, number];
	readonly target: readonly [number, number, number];
	readonly zoom: number;
	readonly fov?: number;
	readonly up?: readonly [number, number, number];
}

export interface IconRenderLights {
	readonly ambientIntensity: number;
	readonly ambientColor: string;
	readonly sunAzimuth: number;
	readonly sunElevation: number;
	readonly sunIntensity: number;
	readonly sunColor: string;
}

export interface IconRenderMaterial {
	readonly color?: string;
	readonly metalness?: number;
	readonly roughness?: number;
	readonly emissive?: string;
	readonly emissiveIntensity?: number;
}

export interface IconRenderRequest {
	readonly assetUrl: string;
	readonly camera: IconRenderCamera;
	readonly lights: IconRenderLights;
	readonly width: number;
	readonly height: number;
	readonly format: IconRenderFormat;
	readonly shape?: IconRenderShape;
	readonly background?: string;
	readonly shadowEnabled?: boolean;
	readonly material?: IconRenderMaterial;
}

export interface IconRenderResult {
	readonly dataUrl: string;
	readonly svgMarkup?: string;
}

export interface IconRenderPort {
	render(request: IconRenderRequest): Promise<IconRenderResult>;
}
//#endregion 🔖IconRenderPort
//#endregion 🔖icon-render-port
