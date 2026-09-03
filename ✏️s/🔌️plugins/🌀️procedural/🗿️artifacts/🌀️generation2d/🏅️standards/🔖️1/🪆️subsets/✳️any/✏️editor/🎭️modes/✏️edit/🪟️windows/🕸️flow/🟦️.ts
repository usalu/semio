/** 🕸️ Generation2d editor — Flow window: typed twin of `🦀️.rs`'s view-model. Mirrors the
 * pane's `render(document: &Generation2dSnapshot, config: &Generation2dConfig, session:
 * &FlowEvalSession)` boundary — the editable node-graph scene payload a mutation-capable surface
 * carries (absent from the viewer's read-only twin, see `👁️viewer/…/🟦️.ts`). */

export interface Generation2dFlowViewModel {
  windowKindId: "generation2d-main";
  bodyKey: "generation2d.play.main";
  surfaceId: "generation2d.play.main";
  editable: true;
}

export const GENERATION2D_PLAY_WINDOW_MAIN = "generation2d-main" as const;
export const GENERATION2D_PLAY_BODY_MAIN = "generation2d.play.main" as const;
