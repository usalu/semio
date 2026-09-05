/** ✏️ EN 1995 editor — subset-level typed twin. Mirrors the editor manifest's mode/
 * window vocabulary; namespaced re-exports (not a blanket `export *`) since every window twin
 * independently declares its own same-shaped `*ViewModel` interface. */

export const EN1995_EDITOR_DIALECT = { artifactKind: "s.norm.en1995", standard: "1", subset: "*" } as const;

export const EN1995_EDIT_MODE_ID = "edit" as const;

export * as inputsWindow from "./🎭️modes/✏️edit/🪟️windows/📥️inputs/🟦️";
export * as resultsWindow from "./🎭️modes/✏️edit/🪟️windows/📊️results/🟦️";
