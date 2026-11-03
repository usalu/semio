/** 🚧 `change-self-weight-assumed-gk` wire twin: the leaf payload `ChangeSelfWeightAssumedGk`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSelfWeightAssumedGk {
  id: string;
  newAssumedGk: number;
}

export const parseChangeSelfWeightAssumedGk: NormWireReader<ChangeSelfWeightAssumedGk> = normWireObject<ChangeSelfWeightAssumedGk>({ id: normWireRequired(normWireString), newAssumedGk: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
