/** 💡️ Din4108 inference schema — document outline (field/section list + entry count). */

export interface Din4108Outline {
  sectionOutline: string[];
  fieldCount: number;
  entryCount: number;
}

export interface Din4108Inference {
  /** @state inferred */
  outline: Din4108Outline;
}
