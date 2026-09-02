/** 👁️ DIN V 18599 viewer — subset-level typed twin. Read-only counterpart of
 * `✏️editor/🟦️.ts`: mirrors the viewer manifest's mode/window vocabulary, no mutation-shaped
 * exports (no command payload types, no config schema beyond the framework's own empty config). */

export const DIN18599_VIEWER_DIALECT = { artifactKind: "s.norm.din18599", standard: "1", subset: "*" } as const;

export const DIN18599_VIEW_MODE_ID = "view" as const;

export * from "./🎭️modes/👁️view/🪟️windows/📊️report/🟦️component";
