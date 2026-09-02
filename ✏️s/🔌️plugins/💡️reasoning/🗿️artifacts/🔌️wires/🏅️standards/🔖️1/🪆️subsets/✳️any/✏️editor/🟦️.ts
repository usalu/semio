/** ✏️ Wires editor — subset-level typed twin. Re-exports the canvas window's typed view-model binding
 * so a host-side TS consumer has one import surface for the whole editor manifest, mirroring
 * `🦀️.rs`'s `create_wires_app()` stitching the mode/window module together. */

export const WIRES_EDITOR_DIALECT = { artifactKind: "s.reasoning.wires", standard: "1", subset: "*" } as const;

export const WIRES_PLAY_MODE_EDIT = "edit" as const;

export * as canvasWindow from "./🎭️modes/✏️edit/🪟️windows/🕸️canvas/🟦️";
