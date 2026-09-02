/** 👁️ Procedural2d editor — Preview window (edit mode): typed twin of `🦀️.rs`'s view-model.
 * Mirrors `render(document: &Procedural2dSnapshot, config: &Procedural2dConfig, session:
 * &FlowEvalSession)` — the evaluated 2D canvas scene, overlaying a schematic wire-mode node box per
 * widget when `config.showMode === "wire"`. */

export interface Procedural2dEditPreviewViewModel {
  windowKindId: "procedural2d-preview";
  bodyKey: "procedural2d.play.preview";
  surfaceId: "procedural2d.play.preview";
  showMode: "preview" | "generate" | "wire";
}

export const PROCEDURAL2D_PLAY_WINDOW_PREVIEW = "procedural2d-preview" as const;
export const PROCEDURAL2D_PLAY_BODY_PREVIEW = "procedural2d.play.preview" as const;
