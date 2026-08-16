/** ▶️ Forms editor — Try window: typed twin of `🦀️component.rs`'s view boundary. Mirrors
 * `render(spec: &FormsSnapshot, config: &FormsConfig, labels: &FormsLabels) -> UiNode`'s wizard-style
 * form-filling preview (a `Canvas2d` surface — no dedicated world/block-list surface id, unlike the
 * sibling Blueprint window). */

/** ▶️ The Try window's typed view-model — mirrors the Rust `render()` boundary's inputs. */
export interface FormsTryViewModel {
  windowKindId: "forms-try";
  bodyKey: "forms.play.try";
  currentStepIndex: number;
  tryValuesJson: string;
}

export const FORMS_PLAY_TRY_WINDOW_KIND_ID = "forms-try" as const;
export const FORMS_PLAY_TRY_BODY_KEY = "forms.play.try" as const;
