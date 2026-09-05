/** 💡️ En1992 inference schema — document outline (field/section list + entry count). */

export interface En1992Outline {
  sectionOutline: string[];
  fieldCount: number;
  entryCount: number;
}

export interface En1992Inference {
  /** @derived */
  outline: En1992Outline;
}
