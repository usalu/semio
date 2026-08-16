/** 🕸️ Wires viewer — Canvas window: typed twin of `🦀️component.rs`'s view-model. Read-only mirror of
 * the 2D canvas scene `render()` produces — no mutation-shaped fields (no drag/add/delete command
 * channel), matching the viewer's `ViewEmit`-only contract. */

/** 👁️ One board-graph layer entry serialized onto the canvas-2d scene (`layers_json`) — a raw node or
 * edge record read straight off the wires board fixture. */
export interface WiresViewCanvasLayer {
  id: string;
  kind?: string;
  [key: string]: unknown;
}

/** 👁️ The Canvas window's typed view-model — the TS mirror of the Rust `render()` boundary's inputs
 * (a bare `WiresSnapshot`, no config/runtime/utility state: a viewer has none of those). */
export interface WiresViewCanvasViewModel {
  windowKindId: "reasoning-wires-view-composite";
  bodyKey: "reasoning.wires.view.composite";
  surfaceId: "reasoning.wires.view.composite";
  cameraX: number;
  cameraY: number;
  zoom: number;
  layers: WiresViewCanvasLayer[];
}

export const WIRES_VIEW_WINDOW_CANVAS = "reasoning-wires-view-composite" as const;
export const WIRES_VIEW_BODY_CANVAS = "reasoning.wires.view.composite" as const;
export const WIRES_VIEW_CANVAS_SURFACE_ID = "reasoning.wires.view.composite" as const;
