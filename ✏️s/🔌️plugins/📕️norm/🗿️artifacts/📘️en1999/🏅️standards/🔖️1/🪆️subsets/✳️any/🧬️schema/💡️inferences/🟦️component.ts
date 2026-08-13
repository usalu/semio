/** 💡️ En1999 inference schema — document outline (field/section list + entry count). */

export interface En1999Outline {
  sectionOutline: string[];
  fieldCount: number;
  entryCount: number;
}

export interface En1999Inference {
  /** @derived */
  outline: En1999Outline;
}
