/** 🚶️ `change-member-bridge-crowd` wire twin: the leaf payload `ChangeMemberBridgeCrowd`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberBridgeCrowd {
  memberId: string;
  newValue: number;
}

export const parseChangeMemberBridgeCrowd: NormWireReader<ChangeMemberBridgeCrowd> = normWireObject<ChangeMemberBridgeCrowd>({ memberId: normWireRequired(normWireString), newValue: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
