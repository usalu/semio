/** 👁️ Procedural2d editor — Preview window (generate mode): typed twin of `🦀️component.rs`'s
 * view-model. Mirrors `render(config: &Procedural2dConfig, labels: &Procedural2dLabels)` — the
 * evaluated generation output preview, hinting when no generation has been evaluated yet. */

export interface Procedural2dGeneratePreviewViewModel {
  windowKindId: "procedural2d-generate-preview";
  bodyKey: "procedural2d.play.generate-preview";
  surfaceId: "procedural2d.play.generate-preview";
  generationPreviewText: string | null;
}

export const PROCEDURAL2D_PLAY_WINDOW_GENERATE_PREVIEW = "procedural2d-generate-preview" as const;
export const PROCEDURAL2D_PLAY_BODY_GENERATE_PREVIEW = "procedural2d.play.generate-preview" as const;
