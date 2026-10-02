/** 🌉️ `change-member-bridge-n-obs` wire twin: the leaf payload `ChangeMemberBridgeNObs`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeMemberBridgeNObs {
  memberId: string;
  newValue: number;
}

export const parseChangeMemberBridgeNObs: NormWireReader<ChangeMemberBridgeNObs> = normWireObject<ChangeMemberBridgeNObs>({ memberId: normWireRequired(normWireString), newValue: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
