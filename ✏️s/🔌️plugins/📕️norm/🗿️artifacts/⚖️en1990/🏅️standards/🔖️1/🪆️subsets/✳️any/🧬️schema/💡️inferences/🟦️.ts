/** 💡️ `En1990Inference` wire twin: the derived reading of the document, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireBoolean, normWireInteger, normWireNumber, normWireObject, normWireRange, normWireRef, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface En1990Inference {
  outline: En1990Outline;
}

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

export const parseEn1990Inference: NormWireReader<En1990Inference> = normWireObject<En1990Inference>({ outline: normWireRequired(normWireRef(() => parseEn1990Outline)) });
export const parseEn1990Outline: NormWireReader<En1990Outline> = normWireObject<En1990Outline>({ sectionOutline: normWireRequired(normWireArray(normWireString)), fieldCount: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), entryCount: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), checkCount: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), passCount: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), allPass: normWireRequired(normWireBoolean), governingClause: normWireRequired(normWireString), governingUtilization: normWireRequired(normWireNumber) });
