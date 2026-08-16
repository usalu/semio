/** 📥️ EN 1997 editor — inputs window: typed twin of `🦀️component.rs`'s view-model
 * boundary (the raw compliance document, rendered as pretty-printed JSON). */

export interface En1997InputsViewModel {
  windowKindId: "norm-en1997-inputs";
  bodyKey: "norm.en1997.play.inputs";
  documentJson: string;
}

export const EN1997_INPUTS_WINDOW_KIND_ID = "norm-en1997-inputs" as const;
export const EN1997_INPUTS_BODY_KEY = "norm.en1997.play.inputs" as const;
