/** 📥️ VDI 3805 editor — inputs window: typed twin of `🦀️component.rs`'s view-model
 * boundary (the raw compliance document, rendered as pretty-printed JSON). */

export interface Vdi3805InputsViewModel {
  windowKindId: "norm-vdi3805-inputs";
  bodyKey: "norm.vdi3805.play.inputs";
  documentJson: string;
}

export const VDI3805_INPUTS_WINDOW_KIND_ID = "norm-vdi3805-inputs" as const;
export const VDI3805_INPUTS_BODY_KEY = "norm.vdi3805.play.inputs" as const;
