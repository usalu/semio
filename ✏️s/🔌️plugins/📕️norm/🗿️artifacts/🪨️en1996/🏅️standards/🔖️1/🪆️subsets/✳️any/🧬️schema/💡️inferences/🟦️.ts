/** 💡️ `En1996Inference` wire twin: the derived reading of the document, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireInteger, normWireObject, normWireRange, normWireRef, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface En1996Inference {
  outline: En1996Outline;
}

export interface En1996Outline {
  sectionOutline: string[];
  fieldCount: number;
  entryCount: number;
}

export const parseEn1996Inference: NormWireReader<En1996Inference> = normWireObject<En1996Inference>({ outline: normWireRequired(normWireRef(() => parseEn1996Outline)) });
export const parseEn1996Outline: NormWireReader<En1996Outline> = normWireObject<En1996Outline>({ sectionOutline: normWireRequired(normWireArray(normWireString)), fieldCount: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), entryCount: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
