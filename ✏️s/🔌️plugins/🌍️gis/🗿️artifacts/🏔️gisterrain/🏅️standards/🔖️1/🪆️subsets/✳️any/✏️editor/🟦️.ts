/** ⛰️ GIS terrain editor — subset-level typed twin. Namespaced re-export (not a blanket `export *`)
 * because window-level twins may independently declare same-named interfaces. */

export const GISTERRAIN_EDITOR_DIALECT = { artifactKind: "s.gis.gisterrain", standard: "1", subset: "*" } as const;

export const GIS3D_PLAY_MODE_VIEW = "view" as const;

export * as terrainWindow from "./🎭️modes/👁️view/🪟️windows/🏔️terrain/🟦️component";
