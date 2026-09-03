/** 👁️ Generation2d editor — Preview window (edit mode): typed twin of `🦀️.rs`'s view-model.
 * Mirrors `render(document: &Generation2dSnapshot, config: &Generation2dConfig, session:
 * &FlowEvalSession)` — the evaluated 2D canvas scene, overlaying a schematic wire-mode node box per
 * widget when `config.showMode === "wire"`. */

export interface Generation2dEditPreviewViewModel {
  windowKindId: "generation2d-preview";
  bodyKey: "generation2d.play.preview";
  surfaceId: "generation2d.play.preview";
  showMode: "preview" | "generate" | "wire";
}

export const GENERATION2D_PLAY_WINDOW_PREVIEW = "generation2d-preview" as const;
export const GENERATION2D_PLAY_BODY_PREVIEW = "generation2d.play.preview" as const;
