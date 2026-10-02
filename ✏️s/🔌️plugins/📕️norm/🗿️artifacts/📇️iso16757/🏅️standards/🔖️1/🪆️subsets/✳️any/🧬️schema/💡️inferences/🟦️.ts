/** 💡️ `Iso16757Inference` wire twin: the derived reading of the document, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireInteger, normWireObject, type NormWireReader, normWireRef, normWireRequired, normWireString } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface Iso16757Inference {
  outline: Iso16757Outline;
}

export interface Iso16757Outline {
  sectionOutline: string[];
  fieldCount: number;
  entryCount: number;
}

export const parseIso16757Inference: NormWireReader<Iso16757Inference> = normWireObject<Iso16757Inference>({ outline: normWireRequired(normWireRef(() => parseIso16757Outline)) });
export const parseIso16757Outline: NormWireReader<Iso16757Outline> = normWireObject<Iso16757Outline>({ sectionOutline: normWireRequired(normWireArray(normWireString)), fieldCount: normWireRequired(normWireInteger), entryCount: normWireRequired(normWireInteger) });
