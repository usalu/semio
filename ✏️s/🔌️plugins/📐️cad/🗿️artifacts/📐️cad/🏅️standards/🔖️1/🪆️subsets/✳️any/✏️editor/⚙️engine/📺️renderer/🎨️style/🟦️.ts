/** 🎨️ The single product-aligned spatial scene colour palette with its CSS-resolved cache, plus the CAD field class constants.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | renderer}, which re-exports every public name below so import specifiers stay unchanged.
 */

import { ephemeralBox } from "@semio-tech/framework";
// #region 🔌️Adapters
import { clearColorResolveCache, resolveSemanticColorHex, tokenHex } from "@semio-tech/framework/ui/styling";
// #endregion 🔌️Adapters
import { THREE } from "../🔌️ports/🟦️.ts";

// #region 🖼️DisplayPrimitives
// #region 🎨️SpatialSceneColors
/** 🎨️ Resolved product palette for spatial canvas materials (no ad-hoc hex in hosts). */
export interface SpatialSceneColorPalette {
  readonly canvas: string;
  readonly accent: string;
  readonly accentEmissive: string;
  readonly accentSecondary: string;
  readonly accentSecondaryEmissive: string;
  readonly foreground: string;
  readonly muted: string;
  readonly mutedEmissive: string;
  readonly selected: string;
  readonly selectedEmissive: string;
  readonly hovered: string;
  readonly hoveredEmissive: string;
  readonly vertex: string;
  readonly vertexEmissive: string;
  readonly edge: string;
  readonly edgeEmissive: string;
  readonly object: string;
  readonly objectEmissive: string;
  readonly face: string;
  readonly faceEmissive: string;
  readonly gridMajor: string;
  readonly gridMinor: string;
  readonly groundPlane: string;
  readonly archived: string;
  readonly archivedEmissive: string;
  readonly ghost: string;
  readonly committed: string;
  readonly committedEmissive: string;
  readonly committedWire: string;
  readonly dimension: string;
  readonly guide: string;
  readonly construction: string;
  readonly constructionEmissive: string;
}

const SPATIAL_SCENE_COLOR_FALLBACK: SpatialSceneColorPalette = {
  canvas: tokenHex("light-6-7"),
  accent: tokenHex("tertiary"),
  accentEmissive: tokenHex("dark-5-7"),
  accentSecondary: tokenHex("gray"),
  accentSecondaryEmissive: tokenHex("dark"),
  foreground: tokenHex("dark"),
  muted: tokenHex("gray"),
  mutedEmissive: tokenHex("dark-6-7"),
  selected: tokenHex("primary"),
  selectedEmissive: tokenHex("dark-5-7"),
  hovered: tokenHex("secondary"),
  hoveredEmissive: tokenHex("dark-5-7"),
  vertex: tokenHex("dark"),
  vertexEmissive: tokenHex("gray-400"),
  edge: tokenHex("gray-400"),
  edgeEmissive: tokenHex("dark-6-7"),
  object: tokenHex("gray"),
  objectEmissive: tokenHex("dark-6-7"),
  face: tokenHex("gray-600"),
  faceEmissive: tokenHex("gray-300"),
  gridMajor: tokenHex("gray-700"),
  gridMinor: tokenHex("light-gray"),
  groundPlane: tokenHex("gray-700"),
  archived: tokenHex("success"),
  archivedEmissive: tokenHex("dark-8-9"),
  ghost: tokenHex("gray"),
  committed: tokenHex("gray"),
  committedEmissive: tokenHex("dark-6-7"),
  committedWire: tokenHex("light-gray"),
  dimension: tokenHex("dark"),
  guide: tokenHex("gray"),
  construction: tokenHex("tertiary"),
  constructionEmissive: tokenHex("dark-5-7"),
};

function readSpatialCssColor(variable: string, fallbackKey: string): string {
  return resolveSemanticColorHex(variable, fallbackKey);
}

const spatialSceneColorCache = ephemeralBox<SpatialSceneColorPalette | null>("s.plugins.cad.modules.renderer.component.tsx.spatialSceneColorCache", null);

/** 🎨️ Reads `--canvas`, `--accent`, and selection tokens for Three.js materials. */
export function spatialSceneColors(): SpatialSceneColorPalette {
  if (spatialSceneColorCache.current) return spatialSceneColorCache.current;
  const accent = readSpatialCssColor("--accent", "tertiary");
  const accentSecondary = readSpatialCssColor("--accent-secondary", "gray");
  const foreground = readSpatialCssColor("--foreground", "dark");
  const muted = readSpatialCssColor("--muted-foreground", "gray");
  const selected = readSpatialCssColor("--color-changed-selected", "primary");
  const hovered = readSpatialCssColor("--color-changed-hovered", "secondary");
  spatialSceneColorCache.current = {
    canvas: readSpatialCssColor("--canvas", "light-6-7"),
    accent,
    accentEmissive: readSpatialCssColor("--active-base", "dark-5-7"),
    accentSecondary,
    accentSecondaryEmissive: readSpatialCssColor("--hover-panel", "dark"),
    foreground,
    muted,
    mutedEmissive: readSpatialCssColor("--hover-base", "dark-6-7"),
    selected,
    selectedEmissive: readSpatialCssColor("--active-base", "dark-5-7"),
    hovered,
    hoveredEmissive: readSpatialCssColor("--hover-panel", "dark-5-7"),
    vertex: foreground,
    vertexEmissive: readSpatialCssColor("--hover-base", "gray-400"),
    edge: readSpatialCssColor("--border-normal-color", "gray-400"),
    edgeEmissive: readSpatialCssColor("--hover-base", "dark-6-7"),
    object: muted,
    objectEmissive: readSpatialCssColor("--hover-panel", "dark-6-7"),
    face: accentSecondary,
    faceEmissive: readSpatialCssColor("--hover-window", "gray-300"),
    gridMajor: readSpatialCssColor("--border-normal-color", "gray-700"),
    gridMinor: readSpatialCssColor("--muted-foreground", "light-gray"),
    groundPlane: readSpatialCssColor("--border-normal-color", "gray-700"),
    archived: readSpatialCssColor("--success-border", "success"),
    archivedEmissive: readSpatialCssColor("--success-foreground", "dark-8-9"),
    ghost: muted,
    committed: accentSecondary,
    committedEmissive: readSpatialCssColor("--hover-panel", "dark-6-7"),
    committedWire: readSpatialCssColor("--foreground", "light-gray"),
    dimension: foreground,
    guide: muted,
    construction: accent,
    constructionEmissive: readSpatialCssColor("--active-base", "dark-5-7"),
  };
  return spatialSceneColorCache.current;
}

/** 🔄️ Clears cached CSS palette (tests or theme switches). */
export function resetSpatialSceneColorCache(): void {
  spatialSceneColorCache.current = null;
  clearColorResolveCache();
}

function spatialSceneColorToHex(color: string): number {
  return new THREE.Color(color).getHex();
}

export const cadFieldClass = "h-medium w-full bg-transparent text-element";
/** 🕳️ Sentinel Select value standing in for "no enum value set", since Radix Select disallows an empty-string item value. */
// #endregion 🎨️SpatialSceneColors
// #endregion 🖼️DisplayPrimitives
export const ENUM_FIELD_NONE_VALUE = "__none__";
