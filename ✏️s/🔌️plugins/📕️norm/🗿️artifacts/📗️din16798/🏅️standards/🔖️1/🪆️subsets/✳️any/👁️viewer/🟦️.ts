/** 👁️ DIN EN 16798 viewer — subset-level typed twin. Read-only counterpart of
 * `✏️editor/🟦️.ts`: mirrors the viewer manifest's mode/window vocabulary, no mutation-shaped
 * exports (no command payload types, no config schema beyond the framework's own empty config). */

export const DIN16798_VIEWER_DIALECT = { artifactKind: "s.norm.din16798", standard: "1", subset: "*" } as const;

export const DIN16798_VIEW_MODE_ID = "view" as const;

export * from "./🎭️modes/👁️view/🪟️windows/📊️report/🟦️component";
