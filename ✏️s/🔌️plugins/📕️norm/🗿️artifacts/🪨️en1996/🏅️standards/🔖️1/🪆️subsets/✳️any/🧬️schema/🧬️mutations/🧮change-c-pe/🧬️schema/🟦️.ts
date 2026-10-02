/** 🧮 `change-c-pe` wire twin: the leaf payload `ChangeCPe`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeCPe {
  wallIndex: number;
  index: number;
  newCPe: number;
}

export const parseChangeCPe: NormWireReader<ChangeCPe> = normWireObject<ChangeCPe>({ wallIndex: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newCPe: normWireRequired(normWireNumber) });
