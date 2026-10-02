/** 🌉️ `change-member-bridge-b` wire twin: the leaf payload `ChangeMemberBridgeB`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeMemberBridgeB {
  memberId: string;
  newValue: number;
}

export const parseChangeMemberBridgeB: NormWireReader<ChangeMemberBridgeB> = normWireObject<ChangeMemberBridgeB>({ memberId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
