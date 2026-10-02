/** ➖️ `remove-joint` wire twin: the leaf payload `RemoveJoint`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface RemoveJoint {
  index: number;
}

export const parseRemoveJoint: NormWireReader<RemoveJoint> = normWireObject<RemoveJoint>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
