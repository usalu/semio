/** 💡️ `En1999Inference` wire twin: the derived reading of the document, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireInteger, normWireObject, normWireRange, normWireRef, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface En1999Inference {
  outline: En1999Outline;
}

export interface En1999Outline {
  sectionOutline: string[];
  fieldCount: number;
  entryCount: number;
}

export const parseEn1999Inference: NormWireReader<En1999Inference> = normWireObject<En1999Inference>({ outline: normWireRequired(normWireRef(() => parseEn1999Outline)) });
export const parseEn1999Outline: NormWireReader<En1999Outline> = normWireObject<En1999Outline>({ sectionOutline: normWireRequired(normWireArray(normWireString)), fieldCount: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), entryCount: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
