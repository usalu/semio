/** 💡️ En1996 inference schema — document outline (field/section list + entry count). */

export interface En1996Outline {
  sectionOutline: string[];
  fieldCount: number;
  entryCount: number;
}

export interface En1996Inference {
  /** @derived */
  outline: En1996Outline;
}
