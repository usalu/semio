/** ✏️ EN 1992 editor — subset-level typed twin. Mirrors the editor manifest's mode/
 * window vocabulary; namespaced re-exports (not a blanket `export *`) since every window twin
 * independently declares its own same-shaped `*ViewModel` interface. */

export const EN1992_EDITOR_DIALECT = { artifactKind: "s.norm.en1992", standard: "1", subset: "*" } as const;

export const EN1992_EDIT_MODE_ID = "edit" as const;

export * as inputsWindow from "./🎭️modes/✏️edit/🪟️windows/📥️inputs/🟦️component";
export * as resultsWindow from "./🎭️modes/✏️edit/🪟️windows/📊️results/🟦️component";
