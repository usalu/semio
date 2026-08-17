/** 👁️ Flow editor — Generate-mode Preview window: typed twin of `🦀️component.rs`'s view-model.
 * Mirrors the pane's `render(config: &FlowConfig)` boundary — the evaluated output preview text of the
 * active generation. */

/** 👁️ The Preview window's typed view-model — mirrors the Rust `render()` boundary's inputs. */
export interface FlowGeneratePreviewViewModel {
  windowKindId: "flow-generate-preview";
  bodyKey: "flow.play.generate-preview";
  surfaceId: "flow.play.generate-preview";
  /** 👁️ Placeholder text shown until the active generation has actually been evaluated. */
  previewText: string;
}

export const FLOW_PLAY_WINDOW_GENERATE_PREVIEW = "flow-generate-preview" as const;
export const FLOW_PLAY_BODY_GENERATE_PREVIEW = "flow.play.generate-preview" as const;
export const FLOW_PLAY_SURFACE_GENERATE_PREVIEW = "flow.play.generate-preview" as const;
