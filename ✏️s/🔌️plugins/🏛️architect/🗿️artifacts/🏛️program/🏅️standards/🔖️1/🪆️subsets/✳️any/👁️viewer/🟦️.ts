/** 👁️ Architect viewer — subset-level typed twin. Re-exports the Register window's typed view-model
 * binding so a host-side TS consumer has one import surface for the whole viewer manifest, mirroring
 * `🦀️.rs`'s `create_architect_viewer()` stitching the mode/window together. */

export const ARCHITECT_VIEWER_DIALECT = { artifactKind: "s.architect.program", standard: "1", subset: "*" } as const;

export const ARCHITECT_VIEW_MODE_VIEW = "view" as const;

export * as registerWindow from "./🎭️modes/👁️view/🪟️windows/📋️register/🟦️component";
