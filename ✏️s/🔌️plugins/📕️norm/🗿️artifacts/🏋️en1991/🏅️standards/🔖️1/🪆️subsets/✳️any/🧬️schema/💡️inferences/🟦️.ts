/** 💡️ `En1991Inference` wire twin: the derived reading of the document, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireInteger, normWireObject, normWireRange, normWireRef, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface En1991Inference {
  outline: En1991Outline;
}

export interface En1991Outline {
  sectionOutline: string[];
  fieldCount: number;
  entryCount: number;
}

export const parseEn1991Inference: NormWireReader<En1991Inference> = normWireObject<En1991Inference>({ outline: normWireRequired(normWireRef(() => parseEn1991Outline)) });
export const parseEn1991Outline: NormWireReader<En1991Outline> = normWireObject<En1991Outline>({ sectionOutline: normWireRequired(normWireArray(normWireString)), fieldCount: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), entryCount: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
