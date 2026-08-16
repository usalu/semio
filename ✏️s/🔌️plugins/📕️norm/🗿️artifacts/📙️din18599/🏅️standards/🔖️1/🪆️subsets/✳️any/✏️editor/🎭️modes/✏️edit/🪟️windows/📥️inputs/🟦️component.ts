/** 📥️ DIN V 18599 editor — inputs window: typed twin of `🦀️component.rs`'s view-model
 * boundary (the raw compliance document, rendered as pretty-printed JSON). */

export interface Din18599InputsViewModel {
  windowKindId: "norm-din18599-inputs";
  bodyKey: "norm.din18599.play.inputs";
  documentJson: string;
}

export const DIN18599_INPUTS_WINDOW_KIND_ID = "norm-din18599-inputs" as const;
export const DIN18599_INPUTS_BODY_KEY = "norm.din18599.play.inputs" as const;
