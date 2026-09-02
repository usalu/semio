/** 🧱️ Forms editor — Blueprint window: typed twin of `🦀️.rs`'s view boundary. Mirrors
 * `render(spec: &FormsSnapshot, config: &FormsConfig, labels: &FormsLabels) -> UiNode`'s block-list
 * surface — the drag/drop palette-backed canvas authoring the form's step/question tree. */

/** 🧱️ The Blueprint window's typed view-model — mirrors the Rust `render()` boundary's inputs. */
export interface FormsBuilderViewModel {
  windowKindId: "forms-blueprint";
  bodyKey: "forms.play.blueprint";
  surfaceId: "forms.play.blueprint";
}

export const FORMS_PLAY_BLUEPRINT_WINDOW_KIND_ID = "forms-blueprint" as const;
export const FORMS_PLAY_BLUEPRINT_BODY_KEY = "forms.play.blueprint" as const;
export const FORMS_PLAY_BLUEPRINT_SURFACE_ID = "forms.play.blueprint" as const;
