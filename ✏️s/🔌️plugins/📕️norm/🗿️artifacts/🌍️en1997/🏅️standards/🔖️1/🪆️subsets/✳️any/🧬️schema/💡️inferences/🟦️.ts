/** 💡️ `En1997Inference` wire twin: the derived reading of the document, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireInteger, normWireObject, type NormWireReader, normWireRef, normWireRequired, normWireString } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface En1997Inference {
  outline: En1997Outline;
}

export interface En1997Outline {
  sectionOutline: string[];
  fieldCount: number;
  entryCount: number;
}

export const parseEn1997Inference: NormWireReader<En1997Inference> = normWireObject<En1997Inference>({ outline: normWireRequired(normWireRef(() => parseEn1997Outline)) });
export const parseEn1997Outline: NormWireReader<En1997Outline> = normWireObject<En1997Outline>({ sectionOutline: normWireRequired(normWireArray(normWireString)), fieldCount: normWireRequired(normWireInteger), entryCount: normWireRequired(normWireInteger) });
