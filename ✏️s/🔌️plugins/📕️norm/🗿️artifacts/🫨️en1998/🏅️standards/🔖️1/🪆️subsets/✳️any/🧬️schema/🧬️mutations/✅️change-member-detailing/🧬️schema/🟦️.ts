/** ✅️ `change-member-detailing` wire twin: the leaf payload `ChangeMemberDetailing`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireBoolean, normWireInteger, normWireLiteral, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeMemberDetailing {
  mutation: "changeMemberDetailing";
  buildingIndex: number;
  memberIndex: number;
  newDetailingCompatibleWithQ: boolean;
}

export const parseChangeMemberDetailing: NormWireReader<ChangeMemberDetailing> = normWireObject<ChangeMemberDetailing>({ mutation: normWireRequired(normWireLiteral("changeMemberDetailing")), buildingIndex: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), memberIndex: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newDetailingCompatibleWithQ: normWireRequired(normWireBoolean) });
