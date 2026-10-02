/** 💡️ `Din4108Inference` wire twin: the derived reading of the document, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireInteger, normWireObject, type NormWireReader, normWireRef, normWireRequired, normWireString } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface Din4108Inference {
  outline: Din4108Outline;
}

export interface Din4108Outline {
  sectionOutline: string[];
  fieldCount: number;
  entryCount: number;
}

export const parseDin4108Inference: NormWireReader<Din4108Inference> = normWireObject<Din4108Inference>({ outline: normWireRequired(normWireRef(() => parseDin4108Outline)) });
export const parseDin4108Outline: NormWireReader<Din4108Outline> = normWireObject<Din4108Outline>({ sectionOutline: normWireRequired(normWireArray(normWireString)), fieldCount: normWireRequired(normWireInteger), entryCount: normWireRequired(normWireInteger) });
