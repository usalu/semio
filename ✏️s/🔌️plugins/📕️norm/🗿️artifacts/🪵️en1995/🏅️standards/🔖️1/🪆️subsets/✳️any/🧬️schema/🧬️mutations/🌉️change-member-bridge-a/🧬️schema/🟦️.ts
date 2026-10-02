/** 🌉️ `change-member-bridge-a` wire twin: the leaf payload `ChangeMemberBridgeA`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeMemberBridgeA {
  memberId: string;
  newValue: number;
}

export const parseChangeMemberBridgeA: NormWireReader<ChangeMemberBridgeA> = normWireObject<ChangeMemberBridgeA>({ memberId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
