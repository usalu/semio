/** 👁️ Shooting viewer — subset-level typed twin. Read-only counterpart of `✏️editor/🟦️component.ts`:
 * mirrors the viewer manifest's mode/window vocabulary, no mutation-shaped exports (no command payload
 * types, no config schema beyond the framework's own empty config). */

export const SHOOTING_VIEWER_DIALECT = { artifactKind: "s.shooting.shooting", standard: "1", subset: "*" } as const;

export const SHOOTING_VIEW_MODE_ID = "view" as const;

export * from "./🎭️modes/👁️view/🪟️windows/🎥️scene/🟦️component";
