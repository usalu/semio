/** 👁️ Sequence viewer — subset-level typed twin. Re-exports the Main window's typed view-model
 * binding, mirroring `🦀️component.rs`'s `create_sequence_viewer()` stitching the mode/window module
 * together. Genuinely independent of the editor's own surface-root twin (`✏️editor/🟦️component.ts`)
 * — never imports from it. */

export const SEQUENCE_VIEWER_DIALECT = { artifactKind: "s.sequence.sequence", standard: "1", subset: "*" } as const;

export const SEQUENCE_VIEW_MODE_VIEW = "view" as const;

export * as mainWindow from "./🎭️modes/👁️view/🪟️windows/📽️main/🟦️component";
