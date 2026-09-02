/** 📥️ EN 1993 editor — inputs window: typed twin of `🦀️.rs`'s view-model
 * boundary (the raw compliance document, rendered as pretty-printed JSON). */

export interface En1993InputsViewModel {
  windowKindId: "norm-en1993-inputs";
  bodyKey: "norm.en1993.play.inputs";
  documentJson: string;
}

export const EN1993_INPUTS_WINDOW_KIND_ID = "norm-en1993-inputs" as const;
export const EN1993_INPUTS_BODY_KEY = "norm.en1993.play.inputs" as const;
