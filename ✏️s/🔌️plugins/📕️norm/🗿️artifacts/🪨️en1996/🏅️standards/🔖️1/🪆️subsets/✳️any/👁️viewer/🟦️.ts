/** 👁️ EN 1996 viewer — subset-level typed twin. Read-only counterpart of
 * `✏️editor/🟦️.ts`: mirrors the viewer manifest's mode/window vocabulary, no mutation-shaped
 * exports (no command payload types, no config schema beyond the framework's own empty config). */

export const EN1996_VIEWER_DIALECT = { artifactKind: "s.norm.en1996", standard: "1", subset: "*" } as const;

export const EN1996_VIEW_MODE_ID = "view" as const;

export * from "./🎭️modes/👁️view/🪟️windows/📊️report/🟦️";
