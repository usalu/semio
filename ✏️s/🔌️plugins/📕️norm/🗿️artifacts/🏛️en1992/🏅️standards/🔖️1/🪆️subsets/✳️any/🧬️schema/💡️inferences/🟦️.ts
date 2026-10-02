/** 💡️ `En1992Inference` wire twin: the derived reading of the document, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireInteger, normWireObject, type NormWireReader, normWireRef, normWireRequired, normWireString } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface En1992Inference {
  outline: En1992Outline;
}

export interface En1992Outline {
  sectionOutline: string[];
  fieldCount: number;
  entryCount: number;
}

export const parseEn1992Inference: NormWireReader<En1992Inference> = normWireObject<En1992Inference>({ outline: normWireRequired(normWireRef(() => parseEn1992Outline)) });
export const parseEn1992Outline: NormWireReader<En1992Outline> = normWireObject<En1992Outline>({ sectionOutline: normWireRequired(normWireArray(normWireString)), fieldCount: normWireRequired(normWireInteger), entryCount: normWireRequired(normWireInteger) });
