/** 💡️ Vdi3805 inference schema — document outline (field/section list + entry count). */

export interface Vdi3805Outline {
  sectionOutline: string[];
  fieldCount: number;
  entryCount: number;
}

export interface Vdi3805Inference {
  /** @derived */
  outline: Vdi3805Outline;
}
