/** 👁️ EN 1995 viewer — subset-level typed twin. Read-only counterpart of
 * `✏️editor/🟦️component.ts`: mirrors the viewer manifest's mode/window vocabulary, no mutation-shaped
 * exports (no command payload types, no config schema beyond the framework's own empty config). */

export const EN1995_VIEWER_DIALECT = { artifactKind: "s.norm.en1995", standard: "1", subset: "*" } as const;

export const EN1995_VIEW_MODE_ID = "view" as const;

export * from "./🎭️modes/👁️view/🪟️windows/📊️report/🟦️component";
