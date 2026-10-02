/** 🚶️ `change-member-bridge-crowd` wire twin: the leaf payload `ChangeMemberBridgeCrowd`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeMemberBridgeCrowd {
  memberId: string;
  newValue: number;
}

export const parseChangeMemberBridgeCrowd: NormWireReader<ChangeMemberBridgeCrowd> = normWireObject<ChangeMemberBridgeCrowd>({ memberId: normWireRequired(normWireString), newValue: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
