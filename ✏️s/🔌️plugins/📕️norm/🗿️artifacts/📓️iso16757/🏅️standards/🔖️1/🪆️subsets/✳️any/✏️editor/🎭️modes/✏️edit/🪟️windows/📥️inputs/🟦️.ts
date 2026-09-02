/** 📥️ ISO 16757 editor — inputs window: typed twin of `🦀️.rs`'s view-model
 * boundary (the raw compliance document, rendered as pretty-printed JSON). */

export interface Iso16757InputsViewModel {
  windowKindId: "norm-iso16757-inputs";
  bodyKey: "norm.iso16757.play.inputs";
  documentJson: string;
}

export const ISO16757_INPUTS_WINDOW_KIND_ID = "norm-iso16757-inputs" as const;
export const ISO16757_INPUTS_BODY_KEY = "norm.iso16757.play.inputs" as const;
