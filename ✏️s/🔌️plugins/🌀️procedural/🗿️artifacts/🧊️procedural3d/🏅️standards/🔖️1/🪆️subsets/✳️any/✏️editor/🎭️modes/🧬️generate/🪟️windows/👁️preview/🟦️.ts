/** 👁️ Procedural3d editor — Preview window (generate mode): typed twin of `🦀️.rs`'s
 * view-model. Mirrors the pane's `render(fixture: &FlowFixture, generation: &GenerationPlayState,
 * cfg: &Procedural3dConfig, labels: &Procedural3dLabels, activeUtility: &str)` boundary — the
 * tessellated preview of the selected generation's evaluated fixture, or a hint string when nothing
 * is selected/evaluated yet. */

/** ✏️ The generate-mode Preview window's typed view-model. */
export interface Procedural3dGeneratePreviewViewModel {
  windowKindId: "procedural3d-generate-preview";
  bodyKey: "procedural.play.generate-preview";
  surfaceId: "procedural.play.generate-preview";
  /** 🧬️ The selected generation id, or null when no generation is selected. */
  selectedGenerationId: string | null;
  /** 🧬️ The selected generation's evaluated preview JSON (empty until evaluated). */
  previewText: string | null;
  showMode: string;
  activeUtilityId: string | null;
}

export const PROCEDURAL3D_PLAY_GENERATE_PREVIEW_WINDOW_KIND_ID = "procedural3d-generate-preview" as const;
export const PROCEDURAL3D_PLAY_GENERATE_PREVIEW_BODY_KEY = "procedural.play.generate-preview" as const;
export const PROCEDURAL3D_PLAY_GENERATE_PREVIEW_SURFACE_ID = "procedural.play.generate-preview" as const;
