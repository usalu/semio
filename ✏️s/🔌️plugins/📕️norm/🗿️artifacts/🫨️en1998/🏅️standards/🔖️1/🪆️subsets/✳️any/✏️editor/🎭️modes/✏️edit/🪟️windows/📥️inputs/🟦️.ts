/** 📥️ EN 1998 editor — inputs window: typed twin of `🦀️.rs`'s view-model
 * boundary (the raw compliance document, rendered as pretty-printed JSON). */

export interface En1998InputsViewModel {
  windowKindId: "norm-en1998-inputs";
  bodyKey: "norm.en1998.play.inputs";
  documentJson: string;
}

export const EN1998_INPUTS_WINDOW_KIND_ID = "norm-en1998-inputs" as const;
export const EN1998_INPUTS_BODY_KEY = "norm.en1998.play.inputs" as const;
