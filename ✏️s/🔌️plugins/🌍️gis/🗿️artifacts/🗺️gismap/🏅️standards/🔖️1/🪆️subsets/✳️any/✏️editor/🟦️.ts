/** 🗺️ GIS map editor — subset-level typed twin. Namespaced re-export (not a blanket `export *`)
 * because window-level twins may independently declare same-named interfaces. */

export const GISMAP_EDITOR_DIALECT = { artifactKind: "s.gis.gismap", standard: "1", subset: "*" } as const;

export const GIS2D_PLAY_MODE_EDIT = "edit" as const;

export * as mapWindow from "./🎭️modes/✏️edit/🪟️windows/🗺️map/🟦️";
