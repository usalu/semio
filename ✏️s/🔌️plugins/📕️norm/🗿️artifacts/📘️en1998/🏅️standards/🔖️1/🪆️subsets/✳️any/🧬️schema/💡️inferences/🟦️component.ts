/** 💡️ En1998 inference schema — document outline (field/section list + entry count). */

export interface En1998Outline {
  sectionOutline: string[];
  fieldCount: number;
  entryCount: number;
}

export interface En1998Inference {
  /** @state inferred */
  outline: En1998Outline;
}
