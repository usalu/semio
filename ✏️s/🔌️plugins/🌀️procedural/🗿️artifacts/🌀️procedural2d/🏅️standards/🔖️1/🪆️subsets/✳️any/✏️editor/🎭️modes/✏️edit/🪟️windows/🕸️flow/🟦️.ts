/** 🕸️ Procedural2d editor — Flow window: typed twin of `🦀️.rs`'s view-model. Mirrors the
 * pane's `render(document: &Procedural2dSnapshot, config: &Procedural2dConfig, session:
 * &FlowEvalSession)` boundary — the editable node-graph scene payload a mutation-capable surface
 * carries (absent from the viewer's read-only twin, see `👁️viewer/…/🟦️.ts`). */

export interface Procedural2dFlowViewModel {
  windowKindId: "procedural2d-main";
  bodyKey: "procedural2d.play.main";
  surfaceId: "procedural2d.play.main";
  editable: true;
}

export const PROCEDURAL2D_PLAY_WINDOW_MAIN = "procedural2d-main" as const;
export const PROCEDURAL2D_PLAY_BODY_MAIN = "procedural2d.play.main" as const;
