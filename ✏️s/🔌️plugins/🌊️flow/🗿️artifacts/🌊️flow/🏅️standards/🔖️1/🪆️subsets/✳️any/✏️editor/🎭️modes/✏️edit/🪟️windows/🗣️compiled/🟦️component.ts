/** 🗣️ Flow editor — Compiled DAG window: typed twin of `🦀️component.rs`'s view-model. Mirrors the
 * pane's `render(fixture: &FlowSnapshot, config: &FlowConfig, session: &FlowEvalSession)` boundary —
 * the read-only compiled wire literal of the current fixture, rendered as a text-editor scene. */

/** 🗣️ The Compiled DAG window's typed view-model — mirrors the Rust `render()` boundary's inputs. */
export interface FlowCompiledViewModel {
  windowKindId: "flow-compiled-dag";
  bodyKey: "flow.play.compiled-dag";
  surfaceId: "flow.play.compiled-dag";
  language: "wire";
  compiledWireLiteral: string;
}

export const FLOW_PLAY_WINDOW_COMPILED = "flow-compiled-dag" as const;
export const FLOW_PLAY_BODY_COMPILED = "flow.play.compiled-dag" as const;
export const FLOW_PLAY_SURFACE_COMPILED = "flow.play.compiled-dag" as const;
