/** 📥️ EN 1990 editor — inputs window: typed twin of `🦀️component.rs`'s view-model
 * boundary (the raw compliance document, rendered as pretty-printed JSON). */

export interface En1990InputsViewModel {
  windowKindId: "norm-en1990-inputs";
  bodyKey: "norm.en1990.play.inputs";
  documentJson: string;
}

export const EN1990_INPUTS_WINDOW_KIND_ID = "norm-en1990-inputs" as const;
export const EN1990_INPUTS_BODY_KEY = "norm.en1990.play.inputs" as const;
