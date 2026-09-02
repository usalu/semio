/** ✏️ DIN V 18599 editor — subset-level typed twin. Mirrors the editor manifest's mode/
 * window vocabulary; namespaced re-exports (not a blanket `export *`) since every window twin
 * independently declares its own same-shaped `*ViewModel` interface. */

export const DIN18599_EDITOR_DIALECT = { artifactKind: "s.norm.din18599", standard: "1", subset: "*" } as const;

export const DIN18599_EDIT_MODE_ID = "edit" as const;

export * as inputsWindow from "./🎭️modes/✏️edit/🪟️windows/📥️inputs/🟦️component";
export * as resultsWindow from "./🎭️modes/✏️edit/🪟️windows/📊️results/🟦️component";
