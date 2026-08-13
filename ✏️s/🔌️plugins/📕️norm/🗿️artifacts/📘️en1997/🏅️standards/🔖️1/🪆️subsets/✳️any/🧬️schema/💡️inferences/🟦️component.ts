/** 💡️ En1997 inference schema — document outline (field/section list + entry count). */

export interface En1997Outline {
  sectionOutline: string[];
  fieldCount: number;
  entryCount: number;
}

export interface En1997Inference {
  /** @derived */
  outline: En1997Outline;
}
