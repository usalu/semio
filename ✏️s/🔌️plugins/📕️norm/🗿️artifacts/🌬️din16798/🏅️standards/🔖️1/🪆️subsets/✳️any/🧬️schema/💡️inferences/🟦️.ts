/** 💡️ `Din16798Inference` wire twin: the derived reading of the document, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireInteger, normWireObject, type NormWireReader, normWireRef, normWireRequired, normWireString } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface Din16798Inference {
  outline: Din16798Outline;
}

export interface Din16798Outline {
  sectionOutline: string[];
  fieldCount: number;
  entryCount: number;
}

export const parseDin16798Inference: NormWireReader<Din16798Inference> = normWireObject<Din16798Inference>({ outline: normWireRequired(normWireRef(() => parseDin16798Outline)) });
export const parseDin16798Outline: NormWireReader<Din16798Outline> = normWireObject<Din16798Outline>({ sectionOutline: normWireRequired(normWireArray(normWireString)), fieldCount: normWireRequired(normWireInteger), entryCount: normWireRequired(normWireInteger) });
