/** 📥️ EN 1994 editor — inputs window: typed twin of `🦀️.rs`'s view-model
 * boundary (the raw compliance document, rendered as pretty-printed JSON). */

export interface En1994InputsViewModel {
  windowKindId: "norm-en1994-inputs";
  bodyKey: "norm.en1994.play.inputs";
  documentJson: string;
}

export const EN1994_INPUTS_WINDOW_KIND_ID = "norm-en1994-inputs" as const;
export const EN1994_INPUTS_BODY_KEY = "norm.en1994.play.inputs" as const;
