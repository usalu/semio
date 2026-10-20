/** 📐️ The pure presentation props of a node: layout spec to CSS, style spec data attributes and the accessibility ARIA props.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | Interpreter}; importers import this module directly.
 */

import { type CSSProperties, type ReactNode } from "../../⚛️react-port/🟦️.ts";
import { uiSpacingLen } from "@semio-tech/ui-styling";
import {
  type AbsoluteLayout,
  type AccessibilitySpec,
  type EdgeSpace,
  type GridLayout,
  type GridTrack,
  type LayoutSpec,
  type LeafLayout,
  type OverlayLayout,
  type ScrollLayout,
  type Sizing,
  type SpaceToken,
  type StackLayout,
  type StyleSpec
} from "@semio-tech/framework";

//#region 🔖️UiInterpreter
//#region LayoutAndStyle
const SPACE_TOKEN_MULTIPLIER: Record<SpaceToken, number> = { none: 0, xs: 1, sm: 2, md: 4, lg: 6, xl: 8, xxl: 12 };

/** 📐️ Resolves a closed `SpaceToken` against the theme's own `--ui-spacing` ramp
 * (`@semio-tech/ui-styling`'s `uiSpacingLen`) — never a raw pixel value shipped over the wire. The
 * per-token multiplier is the one place this renderer decides "how big is `md`"; it can be retuned
 * without touching the contract, which only ever carries the token name. */
function spaceTokenRem(token: SpaceToken): string {
  return uiSpacingLen(SPACE_TOKEN_MULTIPLIER[token]);
}

function edgeSpaceToPadding(edge: EdgeSpace): string {
  if ("all" in edge) return spaceTokenRem(edge.all);
  if ("symmetric" in edge) return `${spaceTokenRem(edge.symmetric.vertical)} ${spaceTokenRem(edge.symmetric.horizontal)}`;
  return `${spaceTokenRem(edge.each.top)} ${spaceTokenRem(edge.each.right)} ${spaceTokenRem(edge.each.bottom)} ${spaceTokenRem(edge.each.left)}`;
}

function sizingToCss(sizing: Sizing): string | undefined {
  if (sizing === "hug") return undefined;
  if (sizing === "fill") return "100%";
  if (typeof sizing === "object" && "fixed" in sizing) return spaceTokenRem(sizing.fixed);
  return undefined;
}

const ALIGN_CSS: Record<string, string> = { start: "flex-start", center: "center", end: "flex-end", stretch: "stretch", baseline: "baseline" };
const JUSTIFY_CSS: Record<string, string> = { start: "flex-start", center: "center", end: "flex-end", spaceBetween: "space-between", spaceAround: "space-around", spaceEvenly: "space-evenly" };

function gridTrackToCss(track: GridTrack): string {
  if (track === "auto") return "auto";
  if (track === "minContent") return "min-content";
  if (track === "maxContent") return "max-content";
  if (typeof track === "object" && "fraction" in track) return `${track.fraction}fr`;
  if (typeof track === "object" && "fixed" in track) return spaceTokenRem(track.fixed);
  return "auto";
}

/** 🧬️ Resolves one `LayoutSpec` variant into inline flex/grid/overflow CSS — the renderer-neutral
 * vocabulary's React reading. Every metric traces back to a closed enum; nothing here is a value that
 * came off the wire directly. */
export function layoutSpecStyle(layout: LayoutSpec): CSSProperties {
  switch (layout.kind) {
    case "leaf": {
      const l = layout as LeafLayout & { kind: "leaf" };
      return { width: sizingToCss(l.width), height: sizingToCss(l.height), minWidth: 0, minHeight: 0 };
    }
    case "stack": {
      const l = layout as StackLayout & { kind: "stack" };
      return {
        display: "flex",
        flexDirection: l.axis === "horizontal" ? "row" : "column",
        gap: spaceTokenRem(l.gap),
        padding: edgeSpaceToPadding(l.padding),
        alignItems: ALIGN_CSS[l.align],
        justifyContent: JUSTIFY_CSS[l.justify],
        flex: l.grow ? "1 1 auto" : undefined,
        flexWrap: l.wrap ? "wrap" : "nowrap",
        minWidth: 0,
        minHeight: 0,
      };
    }
    case "grid": {
      const l = layout as GridLayout & { kind: "grid" };
      return {
        display: "grid",
        gridTemplateColumns: l.columns.map(gridTrackToCss).join(" ") || undefined,
        gridTemplateRows: l.rows.map(gridTrackToCss).join(" ") || undefined,
        columnGap: spaceTokenRem(l.columnGap),
        rowGap: spaceTokenRem(l.rowGap),
        padding: edgeSpaceToPadding(l.padding),
        alignItems: ALIGN_CSS[l.align],
        justifyContent: JUSTIFY_CSS[l.justify],
        minWidth: 0,
        minHeight: 0,
      };
    }
    case "overlay": {
      const l = layout as OverlayLayout & { kind: "overlay" };
      return { position: "relative", padding: edgeSpaceToPadding(l.inset) };
    }
    case "scroll": {
      const l = layout as ScrollLayout & { kind: "scroll" };
      const axes = l.axes;
      return {
        overflowX: axes === "horizontal" || axes === "both" ? "auto" : "hidden",
        overflowY: axes === "vertical" || axes === "both" ? "auto" : "hidden",
        padding: edgeSpaceToPadding(l.padding),
        width: sizingToCss(l.sizing),
        minWidth: 0,
        minHeight: 0,
      };
    }
    case "absolute": {
      const l = layout as AbsoluteLayout & { kind: "absolute" };
      return { position: "absolute", width: sizingToCss(l.sizingWidth), height: sizingToCss(l.sizingHeight) };
    }
    default:
      return { minWidth: 0, minHeight: 0 };
  }
}

/** 🎨️ `StyleSpec` is token-only — never a raw color/pixel. This renderer's first-pass reading exposes
 * every token as a `data-*` attribute rather than guessing a color mapping tokens.json does not yet
 * define a ramp for (flagged by this contract's own upstream packet reports); a theme stylesheet
 * targets `[data-tone="danger"]` etc. Swapping this for direct CSS-variable resolution later is a
 * pure addition, not a wire change. */
export function styleSpecDataAttributes(style: StyleSpec): Record<string, string> {
  return {
    "data-variant": style.variant ?? "solid",
    "data-size": style.size ?? "md",
    "data-density": style.density ?? "standard",
    "data-tone": style.tone ?? "neutral",
    "data-emphasis": style.emphasis ?? "regular",
  };
}
//#endregion LayoutAndStyle

//#region Accessibility
/** ♿️ `AccessibilitySpec` → real ARIA props, plus an optional rendered visually-hidden description
 * span (its id feeds `aria-describedby`) — no `role` here, the semantic role comes from `Component`
 * itself (a `Component::Button` is a button on every renderer; see `🦀️accessibility.rs`'s own doc). */
export function accessibilityAriaProps(spec: AccessibilitySpec, idBase: string): { readonly props: Record<string, unknown>; readonly describedBy?: ReactNode } {
  const describedById = spec.description ? `${idBase}-desc` : undefined;
  const props: Record<string, unknown> = {
    "aria-label": spec.label ?? undefined,
    "aria-describedby": describedById,
    "aria-live": spec.live && spec.live !== "off" ? spec.live : undefined,
    "aria-keyshortcuts": spec.shortcut ?? undefined,
    "aria-hidden": spec.hidden ? true : undefined,
  };
  const describedBy = describedById ? (
    <span id={describedById} className="sr-only">
      {spec.description}
    </span>
  ) : undefined;
  return { props, describedBy };
}
//#endregion Accessibility
//#endregion 🔖️UiInterpreter
