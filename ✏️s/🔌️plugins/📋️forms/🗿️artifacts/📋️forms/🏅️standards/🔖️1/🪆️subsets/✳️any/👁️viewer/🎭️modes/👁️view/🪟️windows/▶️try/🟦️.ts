/** ▶️ Forms viewer — Try window: typed twin of `🦀️.rs`'s view boundary. Mirrors
 * `render(document: &FormsSnapshot) -> UiNode`'s read-only, flat form-fill preview — no per-session
 * config, no wizard step state (absent entirely from the editor's own typed twin, see
 * `✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🟦️.ts`). */

/** ▶️ The Try window's typed view-model — mirrors the Rust `render()` boundary's sole input. */
export interface FormsViewTryViewModel {
  windowKindId: "forms-view-try";
  bodyKey: "forms.view.try";
}

export const FORMS_VIEW_TRY_WINDOW_KIND_ID = "forms-view-try" as const;
export const FORMS_VIEW_TRY_BODY_KEY = "forms.view.try" as const;
