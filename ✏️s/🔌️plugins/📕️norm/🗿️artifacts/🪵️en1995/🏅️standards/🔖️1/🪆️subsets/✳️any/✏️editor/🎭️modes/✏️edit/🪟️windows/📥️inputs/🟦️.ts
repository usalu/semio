/** 📥️ EN 1995 editor — inputs window: typed twin of `🦀️.rs`'s view-model
 * boundary (the raw compliance document, rendered as pretty-printed JSON). */

export interface En1995InputsViewModel {
  windowKindId: "norm-en1995-inputs";
  bodyKey: "norm.en1995.play.inputs";
  documentJson: string;
}

export const EN1995_INPUTS_WINDOW_KIND_ID = "norm-en1995-inputs" as const;
export const EN1995_INPUTS_BODY_KEY = "norm.en1995.play.inputs" as const;
