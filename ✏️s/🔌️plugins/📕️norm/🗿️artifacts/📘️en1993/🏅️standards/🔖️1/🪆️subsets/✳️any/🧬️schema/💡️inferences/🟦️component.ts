/** 💡️ En1993 inference schema — document outline (field/section list + entry count). */

export interface En1993Outline {
  sectionOutline: string[];
  fieldCount: number;
  entryCount: number;
}

export interface En1993Inference {
  /** @state inferred */
  outline: En1993Outline;
}
