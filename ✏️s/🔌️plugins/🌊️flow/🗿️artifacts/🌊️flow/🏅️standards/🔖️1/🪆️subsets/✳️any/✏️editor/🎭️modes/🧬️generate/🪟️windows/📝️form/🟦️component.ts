/** 📝️ Flow editor — Generate-mode Form window: typed twin of `🦀️component.rs`'s view-model. Mirrors
 * the pane's `render(fixture: &FlowSnapshot, config: &FlowConfig)` boundary — the input form for the
 * active generation, or a placeholder when no generation exists yet. */

/** 📝️ The Form window's typed view-model — mirrors the Rust `render()` boundary's inputs. */
export interface FlowGenerateFormViewModel {
  windowKindId: "flow-generate-form";
  bodyKey: "flow.play.generate-form";
  /** 📝️ `null` renders the "Add a generation" placeholder copy instead of the form body. */
  activeGenerationId: string | null;
  values: Record<string, unknown>;
}

export const FLOW_PLAY_WINDOW_GENERATE_FORM = "flow-generate-form" as const;
export const FLOW_PLAY_BODY_GENERATE_FORM = "flow.play.generate-form" as const;
