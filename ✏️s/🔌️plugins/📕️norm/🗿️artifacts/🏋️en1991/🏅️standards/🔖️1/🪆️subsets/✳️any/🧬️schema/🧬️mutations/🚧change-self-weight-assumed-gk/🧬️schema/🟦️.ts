/** 🚧 `change-self-weight-assumed-gk` wire twin: the leaf payload `ChangeSelfWeightAssumedGk`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeSelfWeightAssumedGk {
  index: number;
  newAssumedGk: number;
}

export const parseChangeSelfWeightAssumedGk: NormWireReader<ChangeSelfWeightAssumedGk> = normWireObject<ChangeSelfWeightAssumedGk>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newAssumedGk: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
