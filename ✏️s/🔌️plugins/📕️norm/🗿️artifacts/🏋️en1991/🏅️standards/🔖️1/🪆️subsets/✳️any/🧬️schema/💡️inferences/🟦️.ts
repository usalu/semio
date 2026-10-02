/** 💡️ `En1991Inference` wire twin: the derived reading of the document, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireInteger, normWireObject, type NormWireReader, normWireRef, normWireRequired, normWireString } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface En1991Inference {
  outline: En1991Outline;
}

export interface En1991Outline {
  sectionOutline: string[];
  fieldCount: number;
  entryCount: number;
}

export const parseEn1991Inference: NormWireReader<En1991Inference> = normWireObject<En1991Inference>({ outline: normWireRequired(normWireRef(() => parseEn1991Outline)) });
export const parseEn1991Outline: NormWireReader<En1991Outline> = normWireObject<En1991Outline>({ sectionOutline: normWireRequired(normWireArray(normWireString)), fieldCount: normWireRequired(normWireInteger), entryCount: normWireRequired(normWireInteger) });
