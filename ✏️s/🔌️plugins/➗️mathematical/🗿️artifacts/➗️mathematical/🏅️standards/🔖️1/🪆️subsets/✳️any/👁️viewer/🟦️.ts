/** 👁️ Mathematical viewer — subset-level typed twin. Read-only counterpart of
 * `✏️editor/🟦️.ts`: mirrors the viewer manifest's mode/window vocabulary, no mutation-shaped
 * exports (no command payload types, no config schema beyond the framework's own empty config). */

export const MATHEMATICAL_VIEWER_DIALECT = { artifactKind: "s.mathematical.mathematical", standard: "1", subset: "*" } as const;

export const MATH_VIEW_MODE_VIEW = "view" as const;

export * from "./🎭️modes/👁️view/🪟️windows/📐️geometry/🟦️";
