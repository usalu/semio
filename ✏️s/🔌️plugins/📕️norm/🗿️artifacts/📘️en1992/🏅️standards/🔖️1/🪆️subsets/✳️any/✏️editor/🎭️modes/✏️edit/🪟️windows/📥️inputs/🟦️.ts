/** 📥️ EN 1992 editor — inputs window: typed twin of `🦀️.rs`'s view-model
 * boundary (the raw compliance document, rendered as pretty-printed JSON). */

export interface En1992InputsViewModel {
  windowKindId: "norm-en1992-inputs";
  bodyKey: "norm.en1992.play.inputs";
  documentJson: string;
}

export const EN1992_INPUTS_WINDOW_KIND_ID = "norm-en1992-inputs" as const;
export const EN1992_INPUTS_BODY_KEY = "norm.en1992.play.inputs" as const;
