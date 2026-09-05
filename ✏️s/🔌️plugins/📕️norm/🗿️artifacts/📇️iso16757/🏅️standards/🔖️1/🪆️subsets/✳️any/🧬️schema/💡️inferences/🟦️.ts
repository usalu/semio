/** 💡️ Iso16757 inference schema — document outline (field/section list + entry count). */

export interface Iso16757Outline {
  sectionOutline: string[];
  fieldCount: number;
  entryCount: number;
}

export interface Iso16757Inference {
  /** @derived */
  outline: Iso16757Outline;
}
