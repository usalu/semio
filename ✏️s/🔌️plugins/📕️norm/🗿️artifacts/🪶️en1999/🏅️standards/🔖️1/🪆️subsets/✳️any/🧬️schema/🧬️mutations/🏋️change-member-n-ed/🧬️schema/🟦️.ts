/** 🏋️ `change-member-n-ed` wire twin: the leaf payload `ChangeMemberNEd`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeMemberNEd {
  mutation: "changeMemberNEd";
  memberId: string;
  actionId: string;
  newNK: number;
}

export const parseChangeMemberNEd: NormWireReader<ChangeMemberNEd> = normWireObject<ChangeMemberNEd>({ mutation: normWireRequired(normWireLiteral("changeMemberNEd")), memberId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newNK: normWireRequired(normWireNumber) });
