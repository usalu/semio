/** 💡️ Din18599 inference schema — document outline (field/section list + entry count). */

export interface Din18599Outline {
  sectionOutline: string[];
  fieldCount: number;
  entryCount: number;
}

export interface Din18599Inference {
  /** @derived */
  outline: Din18599Outline;
}
