/** 🕸️ Wires editor — Canvas window: typed twin of `🦀️component.rs`'s view-model. Mirrors the pane's
 * `render(board: &DslValue, wires: &DslValue) -> UiNode` boundary — a 2D canvas scene of the live
 * mindmap board (nodes/edges/relationship-edge layers) plus the mutation-capable command channel this
 * surface dispatches through (add/delete/drag/layout), absent entirely from the viewer's read-only
 * twin (see `👁️viewer/…/🕸️canvas/🟦️component.ts`). */

/** ✏️ One board-graph layer entry serialized onto the canvas-2d scene (`layers_json`) — a raw node or
 * edge record from the wires board fixture, shape-free beyond the fields every layer carries. */
export interface WiresCanvasLayer {
  id: string;
  kind?: string;
  [key: string]: unknown;
}

/** ✏️ The Canvas window's typed view-model — mirrors the Rust `render()` boundary's inputs/outputs. */
export interface WiresEditorCanvasViewModel {
  windowKindId: "reasoning-wires-composite";
  bodyKey: "reasoning.wires.composite";
  surfaceId: "reasoning.wires.composite";
  cameraX: number;
  cameraY: number;
  zoom: number;
  layers: WiresCanvasLayer[];
}

export const WIRES_PLAY_WINDOW_CANVAS = "reasoning-wires-composite" as const;
export const WIRES_PLAY_BODY_COMPOSITE = "reasoning.wires.composite" as const;
export const WIRES_PLAY_CANVAS_SURFACE_ID = "reasoning.wires.composite" as const;
