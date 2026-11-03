/** 💡️ `En1997Inference` wire twin: the derived reading of the document, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireBoolean, normWireInteger, normWireNumber, normWireObject, normWireRange, normWireRef, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface En1997Inference {
  outline: En1997Outline;
}

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

export const parseEn1997Inference: NormWireReader<En1997Inference> = normWireObject<En1997Inference>({ outline: normWireRequired(normWireRef(() => parseEn1997Outline)) });
export const parseEn1997Outline: NormWireReader<En1997Outline> = normWireObject<En1997Outline>({ sectionOutline: normWireRequired(normWireArray(normWireString)), fieldCount: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), entryCount: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), checkCount: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), passCount: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), allPass: normWireRequired(normWireBoolean), governingClause: normWireRequired(normWireString), governingUtilization: normWireRequired(normWireNumber), governingSituation: normWireRequired(normWireString), governingApproach: normWireRequired(normWireString) });
