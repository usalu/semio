/** 💡️ En1994 inference schema — document outline (field/section list + entry count). */

export interface En1994Outline {
  sectionOutline: string[];
  fieldCount: number;
  entryCount: number;
}

export interface En1994Inference {
  /** @state inferred */
  outline: En1994Outline;
}
