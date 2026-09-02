/** 👁️ Procedural3d viewer — subset-level typed twin. Re-exports the viewer's single window's typed
 * view-model binding so a host-side TS consumer has one import surface for the whole viewer
 * manifest, mirroring `🦀️.rs`'s `create_procedural3d_viewer()` stitching the mode/window
 * together. MUST NOT import anything from the sibling `✏️editor` surface. */

export const PROCEDURAL3D_VIEWER_DIALECT = { artifactKind: "s.procedural.procedural3d", standard: "1", subset: "*" } as const;

export const PROCEDURAL3D_VIEW_MODE_VIEW = "view" as const;

export * as previewWindow from "./🎭️modes/👁️view/🪟️windows/👁️preview/🟦️component";
