/** 👁️ Generation2d viewer — subset-level typed twin. Read-only counterpart of the editor's
 * `🟦️.ts`: mirrors the viewer manifest's mode/window vocabulary, no mutation-shaped exports
 * (no command payload types, no config schema beyond the framework's own empty config). */

export const GENERATION2D_VIEWER_DIALECT = { artifactKind: "s.procedural.generation2d", standard: "1", subset: "*" } as const;

export const GENERATION2D_VIEW_MODE_ID = "view" as const;

export * from "./🎭️modes/👁️view/🪟️windows/👁️preview/🟦️";
