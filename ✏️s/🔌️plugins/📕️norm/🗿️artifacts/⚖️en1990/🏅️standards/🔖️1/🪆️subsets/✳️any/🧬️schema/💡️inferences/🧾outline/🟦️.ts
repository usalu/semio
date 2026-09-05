/** 🧾 `outline` — document field outline plus governing-clause summary from norm computation. */

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
