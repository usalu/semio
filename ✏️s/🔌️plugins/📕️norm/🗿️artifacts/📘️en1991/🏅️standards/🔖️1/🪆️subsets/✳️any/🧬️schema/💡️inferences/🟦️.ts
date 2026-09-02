/** 💡️ En1991 inference schema — document outline (field/section list + entry count). */

export interface En1991Outline {
  sectionOutline: string[];
  fieldCount: number;
  entryCount: number;
}

export interface En1991Inference {
  /** @derived */
  outline: En1991Outline;
}
