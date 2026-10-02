/** 💡️ `Din18599Inference` wire twin: the derived reading of the document, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireInteger, normWireObject, type NormWireReader, normWireRef, normWireRequired, normWireString } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface Din18599Inference {
  outline: Din18599Outline;
}

export interface Din18599Outline {
  sectionOutline: string[];
  fieldCount: number;
  entryCount: number;
}

export const parseDin18599Inference: NormWireReader<Din18599Inference> = normWireObject<Din18599Inference>({ outline: normWireRequired(normWireRef(() => parseDin18599Outline)) });
export const parseDin18599Outline: NormWireReader<Din18599Outline> = normWireObject<Din18599Outline>({ sectionOutline: normWireRequired(normWireArray(normWireString)), fieldCount: normWireRequired(normWireInteger), entryCount: normWireRequired(normWireInteger) });
