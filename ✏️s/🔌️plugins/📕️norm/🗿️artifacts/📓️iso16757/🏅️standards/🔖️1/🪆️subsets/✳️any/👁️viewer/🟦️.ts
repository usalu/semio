/** 👁️ ISO 16757 viewer — subset-level typed twin. Read-only counterpart of
 * `✏️editor/🟦️.ts`: mirrors the viewer manifest's mode/window vocabulary, no mutation-shaped
 * exports (no command payload types, no config schema beyond the framework's own empty config). */

export const ISO16757_VIEWER_DIALECT = { artifactKind: "s.norm.iso16757", standard: "1", subset: "*" } as const;

export const ISO16757_VIEW_MODE_ID = "view" as const;

export * from "./🎭️modes/👁️view/🪟️windows/📊️report/🟦️component";
