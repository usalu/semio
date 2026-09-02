/** 📥️ DIN EN 16798 editor — inputs window: typed twin of `🦀️.rs`'s view-model
 * boundary (the raw compliance document, rendered as pretty-printed JSON). */

export interface Din16798InputsViewModel {
  windowKindId: "norm-din16798-inputs";
  bodyKey: "norm.din16798.play.inputs";
  documentJson: string;
}

export const DIN16798_INPUTS_WINDOW_KIND_ID = "norm-din16798-inputs" as const;
export const DIN16798_INPUTS_BODY_KEY = "norm.din16798.play.inputs" as const;
