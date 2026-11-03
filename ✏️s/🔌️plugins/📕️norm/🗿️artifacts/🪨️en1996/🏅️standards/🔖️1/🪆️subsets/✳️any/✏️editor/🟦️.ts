/** ✏️ EN 1996 editor — subset-level typed twin. Mirrors the editor manifest's mode/
 * window vocabulary; namespaced re-exports (not a blanket `export *`) since every window twin
 * independently declares its own same-shaped `*ViewModel` interface. */

export const EN1996_EDITOR_DIALECT = { artifactKind: "s.norm.en1996", standard: "1", subset: "*" } as const;

export const EN1996_EDIT_MODE_ID = "edit" as const;

export * as viewportWindow from "./🎭️modes/✏️edit/🪟️windows/🧊️viewport/🟦️";
export * as diagramsWindow from "./🎭️modes/✏️edit/🪟️windows/📈️diagrams/🟦️";
export * as propertiesWindow from "./🎭️modes/✏️edit/🪟️windows/🧾️properties/🟦️";
export * as checksWindow from "./🎭️modes/✏️edit/🪟️windows/✅️checks/🟦️";
