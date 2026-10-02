/** 💡️ `Vdi3805Inference` wire twin: the derived reading of the document, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireInteger, normWireObject, type NormWireReader, normWireRef, normWireRequired, normWireString } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface Vdi3805Inference {
  outline: Vdi3805Outline;
}

export interface Vdi3805Outline {
  sectionOutline: string[];
  fieldCount: number;
  entryCount: number;
}

export const parseVdi3805Inference: NormWireReader<Vdi3805Inference> = normWireObject<Vdi3805Inference>({ outline: normWireRequired(normWireRef(() => parseVdi3805Outline)) });
export const parseVdi3805Outline: NormWireReader<Vdi3805Outline> = normWireObject<Vdi3805Outline>({ sectionOutline: normWireRequired(normWireArray(normWireString)), fieldCount: normWireRequired(normWireInteger), entryCount: normWireRequired(normWireInteger) });
