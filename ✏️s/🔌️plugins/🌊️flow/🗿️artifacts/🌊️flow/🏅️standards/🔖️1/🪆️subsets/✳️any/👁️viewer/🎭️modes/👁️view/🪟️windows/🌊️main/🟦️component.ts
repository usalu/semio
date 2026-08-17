/** 🌊️ Flow viewer — Main window: typed twin of `🦀️component.rs`'s view-model. Mirrors the pane's
 * `render(document: &FlowSnapshot)` boundary — the same node-graph scene shape the mutation-capable
 * Main window renders, always `editable: false` and with no persisted per-session camera/canvas state
 * (`Config = NoConfig`). */

/** 🌊️ The Main window's typed view-model — mirrors the Rust `render()` boundary's inputs. */
export interface FlowViewMainViewModel {
  windowKindId: "flow-view-main";
  bodyKey: "flow.view.main";
  surfaceId: "flow.view.main";
  editable: false;
}

export const FLOW_VIEW_WINDOW_MAIN = "flow-view-main" as const;
export const FLOW_VIEW_BODY_MAIN = "flow.view.main" as const;
export const FLOW_VIEW_SURFACE_MAIN = "flow.view.main" as const;
