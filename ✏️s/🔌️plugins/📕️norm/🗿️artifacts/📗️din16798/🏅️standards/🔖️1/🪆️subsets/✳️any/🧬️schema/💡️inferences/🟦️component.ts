/** 💡️ Din16798 inference schema — document outline (field/section list + entry count). */

export interface Din16798Outline {
  sectionOutline: string[];
  fieldCount: number;
  entryCount: number;
}

export interface Din16798Inference {
  /** @state inferred */
  outline: Din16798Outline;
}
