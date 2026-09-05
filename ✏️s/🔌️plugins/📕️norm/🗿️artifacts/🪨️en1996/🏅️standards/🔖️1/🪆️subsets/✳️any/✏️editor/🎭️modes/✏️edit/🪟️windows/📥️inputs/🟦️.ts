/** 📥️ EN 1996 editor — inputs window: typed twin of `🦀️.rs`'s view-model
 * boundary (the raw compliance document, rendered as pretty-printed JSON). */

export interface En1996InputsViewModel {
  windowKindId: "norm-en1996-inputs";
  bodyKey: "norm.en1996.play.inputs";
  documentJson: string;
}

export const EN1996_INPUTS_WINDOW_KIND_ID = "norm-en1996-inputs" as const;
export const EN1996_INPUTS_BODY_KEY = "norm.en1996.play.inputs" as const;
