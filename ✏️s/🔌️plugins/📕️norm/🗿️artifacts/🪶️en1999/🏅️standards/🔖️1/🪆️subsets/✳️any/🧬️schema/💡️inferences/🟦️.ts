/** 💡️ `En1999Inference` wire twin: the derived reading of the document, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireInteger, normWireObject, type NormWireReader, normWireRef, normWireRequired, normWireString } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface En1999Inference {
  outline: En1999Outline;
}

export interface En1999Outline {
  sectionOutline: string[];
  fieldCount: number;
  entryCount: number;
}

export const parseEn1999Inference: NormWireReader<En1999Inference> = normWireObject<En1999Inference>({ outline: normWireRequired(normWireRef(() => parseEn1999Outline)) });
export const parseEn1999Outline: NormWireReader<En1999Outline> = normWireObject<En1999Outline>({ sectionOutline: normWireRequired(normWireArray(normWireString)), fieldCount: normWireRequired(normWireInteger), entryCount: normWireRequired(normWireInteger) });
