/** 🧾 En1997Outline — section structure + governing design situation. */
export interface En1997Outline {
  sectionOutline: string[];
  fieldCount: number;
  entryCount: number;
  checkCount: number;
  passCount: number;
  allPass: boolean;
  governingClause: string;
  governingUtilization: number;
  governingSituation: string;
  governingApproach: string;
}

export function parseEn1997Outline(value: unknown, _at = "$"): En1997Outline {
  return value as En1997Outline;
}
