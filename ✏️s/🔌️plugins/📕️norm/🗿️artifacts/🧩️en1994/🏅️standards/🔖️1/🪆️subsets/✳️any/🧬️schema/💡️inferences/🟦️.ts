/** 💡️ `En1994Inference` wire twin: the derived reading of the document, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireInteger, normWireObject, type NormWireReader, normWireRef, normWireRequired, normWireString } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface En1994Inference {
  outline: En1994Outline;
}

export interface En1994Outline {
  sectionOutline: string[];
  fieldCount: number;
  entryCount: number;
}

export const parseEn1994Inference: NormWireReader<En1994Inference> = normWireObject<En1994Inference>({ outline: normWireRequired(normWireRef(() => parseEn1994Outline)) });
export const parseEn1994Outline: NormWireReader<En1994Outline> = normWireObject<En1994Outline>({ sectionOutline: normWireRequired(normWireArray(normWireString)), fieldCount: normWireRequired(normWireInteger), entryCount: normWireRequired(normWireInteger) });
