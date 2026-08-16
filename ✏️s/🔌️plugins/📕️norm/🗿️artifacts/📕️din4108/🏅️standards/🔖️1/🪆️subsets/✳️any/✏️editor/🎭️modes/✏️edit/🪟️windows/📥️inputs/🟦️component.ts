/** 📥️ DIN 4108 editor — inputs window: typed twin of `🦀️component.rs`'s view-model
 * boundary (the raw compliance document, rendered as pretty-printed JSON). */

export interface Din4108InputsViewModel {
  windowKindId: "norm-din4108-inputs";
  bodyKey: "norm.din4108.play.inputs";
  documentJson: string;
}

export const DIN4108_INPUTS_WINDOW_KIND_ID = "norm-din4108-inputs" as const;
export const DIN4108_INPUTS_BODY_KEY = "norm.din4108.play.inputs" as const;
