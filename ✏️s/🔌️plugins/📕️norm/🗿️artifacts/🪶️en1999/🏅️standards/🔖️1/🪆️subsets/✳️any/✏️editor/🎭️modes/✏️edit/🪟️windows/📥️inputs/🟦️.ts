/** 📥️ EN 1999 editor — inputs window: typed twin of `🦀️.rs`'s view-model
 * boundary (the raw compliance document, rendered as pretty-printed JSON). */

export interface En1999InputsViewModel {
  windowKindId: "norm-en1999-inputs";
  bodyKey: "norm.en1999.play.inputs";
  documentJson: string;
}

export const EN1999_INPUTS_WINDOW_KIND_ID = "norm-en1999-inputs" as const;
export const EN1999_INPUTS_BODY_KEY = "norm.en1999.play.inputs" as const;
