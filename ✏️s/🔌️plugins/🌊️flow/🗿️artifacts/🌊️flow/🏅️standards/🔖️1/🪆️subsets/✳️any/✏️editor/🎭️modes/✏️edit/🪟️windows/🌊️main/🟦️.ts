/** 🌊️ Flow editor — Main window: typed twin of `🦀️.rs`'s view-model. Mirrors the pane's
 * `render(fixture: &FlowSnapshot, config: &FlowConfig, session: &FlowEvalSession)` boundary — the
 * editable node-graph scene (`SurfaceKind::NodeGraph`, built via `build_node_graph_scene`). */

/** 🌊️ The Main window's typed view-model — mirrors the Rust `render()` boundary's inputs. */
export interface FlowMainViewModel {
  windowKindId: "flow-main";
  bodyKey: "flow.play.main";
  surfaceId: "flow.play.main";
  editable: true;
  cameraX: number;
  cameraY: number;
  cameraZoom: number;
  lodMode: string;
  proximityDistance: number;
  gridVisible: boolean;
  gridSnapEnabled: boolean;
  gridFactor: number;
  previewOffNodeIds: string[];
}

export const FLOW_PLAY_WINDOW_MAIN = "flow-main" as const;
export const FLOW_PLAY_BODY_MAIN = "flow.play.main" as const;
export const FLOW_PLAY_SURFACE_MAIN = "flow.play.main" as const;
