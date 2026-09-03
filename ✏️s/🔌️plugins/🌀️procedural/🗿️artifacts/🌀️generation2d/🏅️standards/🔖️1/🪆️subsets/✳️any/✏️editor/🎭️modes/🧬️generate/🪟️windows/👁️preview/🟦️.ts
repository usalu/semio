/** 👁️ Generation2d editor — Preview window (generate mode): typed twin of `🦀️.rs`'s
 * view-model. Mirrors `render(config: &Generation2dConfig, labels: &Generation2dLabels)` — the
 * evaluated generation output preview, hinting when no generation has been evaluated yet. */

export interface Generation2dGeneratePreviewViewModel {
  windowKindId: "generation2d-generate-preview";
  bodyKey: "generation2d.play.generate-preview";
  surfaceId: "generation2d.play.generate-preview";
  generationPreviewText: string | null;
}

export const GENERATION2D_PLAY_WINDOW_GENERATE_PREVIEW = "generation2d-generate-preview" as const;
export const GENERATION2D_PLAY_BODY_GENERATE_PREVIEW = "generation2d.play.generate-preview" as const;
