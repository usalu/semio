/** 💡️ En1995 inference schema — document outline (field/section list + entry count). */

export interface En1995Outline {
  sectionOutline: string[];
  fieldCount: number;
  entryCount: number;
}

export interface En1995Inference {
  /** @state inferred */
  outline: En1995Outline;
}
