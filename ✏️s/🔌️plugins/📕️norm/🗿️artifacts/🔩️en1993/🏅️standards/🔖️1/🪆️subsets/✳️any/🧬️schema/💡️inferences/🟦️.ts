/** 💡️ `En1993Inference` wire twin: the derived reading of the document, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireInteger, normWireObject, type NormWireReader, normWireRef, normWireRequired, normWireString } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface En1993Inference {
  outline: En1993Outline;
}

export interface En1993Outline {
  sectionOutline: string[];
  fieldCount: number;
  entryCount: number;
}

export const parseEn1993Inference: NormWireReader<En1993Inference> = normWireObject<En1993Inference>({ outline: normWireRequired(normWireRef(() => parseEn1993Outline)) });
export const parseEn1993Outline: NormWireReader<En1993Outline> = normWireObject<En1993Outline>({ sectionOutline: normWireRequired(normWireArray(normWireString)), fieldCount: normWireRequired(normWireInteger), entryCount: normWireRequired(normWireInteger) });
