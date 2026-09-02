/** 👁️ Process 3D viewer — subset-level typed twin. Re-exports the workpiece window's typed
 * view-model binding so a host-side TS consumer has one import surface for the whole viewer
 * manifest, mirroring `🦀️.rs`'s `create_process3d_viewer()` stitching the window module
 * in. MUST NOT import anything from the sibling editor surface's TS twin. */

export const PROCESS3D_VIEWER_DIALECT = { artifactKind: "s.process.process3d", standard: "1", subset: "*" } as const;

export const PROCESS3D_VIEW_MODE_VIEW = "view" as const;

export * from "./🎭️modes/👁️view/🪟️windows/🪚️workpiece/🟦️";
