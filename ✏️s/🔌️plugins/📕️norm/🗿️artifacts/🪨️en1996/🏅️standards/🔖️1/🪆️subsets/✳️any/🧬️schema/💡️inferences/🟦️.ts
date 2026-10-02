/** 💡️ `En1996Inference` wire twin: the derived reading of the document, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireInteger, normWireObject, type NormWireReader, normWireRef, normWireRequired, normWireString } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface En1996Inference {
  outline: En1996Outline;
}

export interface En1996Outline {
  sectionOutline: string[];
  fieldCount: number;
  entryCount: number;
}

export const parseEn1996Inference: NormWireReader<En1996Inference> = normWireObject<En1996Inference>({ outline: normWireRequired(normWireRef(() => parseEn1996Outline)) });
export const parseEn1996Outline: NormWireReader<En1996Outline> = normWireObject<En1996Outline>({ sectionOutline: normWireRequired(normWireArray(normWireString)), fieldCount: normWireRequired(normWireInteger), entryCount: normWireRequired(normWireInteger) });
