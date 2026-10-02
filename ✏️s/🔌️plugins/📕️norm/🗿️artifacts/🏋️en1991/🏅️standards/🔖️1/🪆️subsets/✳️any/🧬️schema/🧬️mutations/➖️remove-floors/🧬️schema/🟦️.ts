/** ➖️ `remove-floors` wire twin: the leaf payload `RemoveFloors`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface RemoveFloors {
  index: number;
}

export const parseRemoveFloors: NormWireReader<RemoveFloors> = normWireObject<RemoveFloors>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
