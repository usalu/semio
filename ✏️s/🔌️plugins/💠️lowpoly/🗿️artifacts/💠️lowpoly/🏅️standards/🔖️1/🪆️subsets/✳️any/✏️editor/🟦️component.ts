/** ✏️ Lowpoly editor — subset-level typed twin. Namespaced re-export of both window twins (not a
 * blanket `export *`, matching the pilot's own convention) so a future collision between the two
 * windows' own local names never becomes ambiguous at this boundary. */

export const LOWPOLY_EDITOR_DIALECT = { artifactKind: "s.lowpoly.lowpoly", standard: "1", subset: "*" } as const;

export const LOWPOLY_EDIT_MODE_ID = "edit" as const;
export const LOWPOLY_PAINT_MODE_ID = "paint" as const;

export * as modelWindow from "./🎭️modes/✏️edit/🪟️windows/🌐️model/🟦️component";
export * as uvWindow from "./🎭️modes/🎨️paint/🪟️windows/🖼️uv/🟦️component";
