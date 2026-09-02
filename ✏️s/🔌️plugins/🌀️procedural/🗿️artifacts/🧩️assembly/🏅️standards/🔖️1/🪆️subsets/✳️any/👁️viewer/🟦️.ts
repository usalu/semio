/** 👁️ Assembly viewer — subset-level typed twin. Read-only counterpart of `✏️editor/🟦️.ts`:
 * mirrors the viewer manifest's mode/window vocabulary, no mutation-shaped exports (no command
 * payload types, no config schema beyond the framework's own empty config). */

export const ASSEMBLY_VIEWER_DIALECT = { artifactKind: "s.assembly", standard: "1", subset: "*" } as const;

export const ASSEMBLY_VIEW_MODE_ID = "view" as const;

export * as structureWindow from "./🎭️modes/👁️view/🪟️windows/🌳️structure/🟦️";
