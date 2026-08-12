/** 💡️ En1990 inference schema — document outline and clause summary. */

export interface En1990Outline {
  sectionOutline: string[];
  fieldCount: number;
  entryCount: number;
  checkCount: number;
  passCount: number;
  allPass: boolean;
  governingClause: string;
  governingUtilization: number;
}

export interface En1990Inference {
  /** @state inferred */
  outline: En1990Outline;
}
