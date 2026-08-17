/** 👁️ Flow viewer — subset-level typed twin. Re-exports the window's typed view-model binding so a
 * host-side TS consumer has one import surface for the whole viewer manifest, mirroring
 * `🦀️component.rs`'s `create_flow_viewer()` stitching the mode/window module together. */

export const FLOW_VIEWER_DIALECT = { artifactKind: "s.flow.flow", standard: "1", subset: "*" } as const;

export const FLOW_VIEW_MODE_VIEW = "view" as const;

export * as mainWindow from "./🎭️modes/👁️view/🪟️windows/🌊️main/🟦️component";
