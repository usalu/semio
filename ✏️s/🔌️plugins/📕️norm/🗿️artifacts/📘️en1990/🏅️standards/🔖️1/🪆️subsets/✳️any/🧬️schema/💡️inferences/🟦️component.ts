/** 💡️ En1990 inference schema — document outline (field/section list + entry count). */

export interface En1990Outline {
  sectionOutline: string[];
  fieldCount: number;
  entryCount: number;
}

export interface En1990Inference {
  /** @state inferred */
  outline: En1990Outline;
}
