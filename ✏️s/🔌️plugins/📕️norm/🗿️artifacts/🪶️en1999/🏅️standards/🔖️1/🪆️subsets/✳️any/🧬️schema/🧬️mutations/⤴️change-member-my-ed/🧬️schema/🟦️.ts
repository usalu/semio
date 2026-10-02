/** ⤴️ `change-member-my-ed` wire twin: the leaf payload `ChangeMemberMYEd`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeMemberMYEd {
  mutation: "changeMemberMYEd";
  memberId: string;
  actionId: string;
  newMYK: number;
}

export const parseChangeMemberMYEd: NormWireReader<ChangeMemberMYEd> = normWireObject<ChangeMemberMYEd>({ mutation: normWireRequired(normWireLiteral("changeMemberMYEd")), memberId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newMYK: normWireRequired(normWireNumber) });
