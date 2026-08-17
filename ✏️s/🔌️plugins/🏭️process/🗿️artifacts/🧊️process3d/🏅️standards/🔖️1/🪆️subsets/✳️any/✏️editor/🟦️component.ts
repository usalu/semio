/** ✏️ Process 3D editor — subset-level typed twin. Re-exports the workpiece window's typed
 * view-model binding so a host-side TS consumer has one import surface for the whole editor
 * manifest, mirroring `🦀️component.rs`'s `create_process3d_app()` stitching the window module in. */

export const PROCESS3D_EDITOR_DIALECT = { artifactKind: "s.process.process3d", standard: "1", subset: "*" } as const;

export const PROCESS3D_MODE_EDIT = "edit" as const;

export * from "./🎭️modes/✏️edit/🪟️windows/🪚️workpiece/🟦️component";
