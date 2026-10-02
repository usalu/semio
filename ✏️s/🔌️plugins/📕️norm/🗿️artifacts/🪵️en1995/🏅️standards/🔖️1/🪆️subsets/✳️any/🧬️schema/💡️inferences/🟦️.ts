/** 💡️ `En1995Inference` wire twin: the derived reading of the document, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireInteger, normWireObject, type NormWireReader, normWireRef, normWireRequired, normWireString } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface En1995Inference {
  outline: En1995Outline;
}

export interface En1995Outline {
  sectionOutline: string[];
  fieldCount: number;
  entryCount: number;
}

export const parseEn1995Inference: NormWireReader<En1995Inference> = normWireObject<En1995Inference>({ outline: normWireRequired(normWireRef(() => parseEn1995Outline)) });
export const parseEn1995Outline: NormWireReader<En1995Outline> = normWireObject<En1995Outline>({ sectionOutline: normWireRequired(normWireArray(normWireString)), fieldCount: normWireRequired(normWireInteger), entryCount: normWireRequired(normWireInteger) });
