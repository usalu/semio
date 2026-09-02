/** 👁️ Wires viewer — subset-level typed twin. Re-exports the canvas window's typed view-model binding
 * so a host-side TS consumer has one import surface for the whole viewer manifest, mirroring
 * `🦀️.rs`'s `create_wires_viewer()` stitching the mode/window module together. */

export const WIRES_VIEWER_DIALECT = { artifactKind: "s.reasoning.wires", standard: "1", subset: "*" } as const;

export const WIRES_VIEW_MODE_VIEW = "view" as const;

export * as canvasWindow from "./🎭️modes/👁️view/🪟️windows/🕸️canvas/🟦️component";
