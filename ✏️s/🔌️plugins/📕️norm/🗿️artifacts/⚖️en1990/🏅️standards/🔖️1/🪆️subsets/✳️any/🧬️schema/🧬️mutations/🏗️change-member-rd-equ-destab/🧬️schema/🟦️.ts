/** 🏗️ `change-member-rd-equ-destab` wire twin: the leaf payload `ChangeMemberRdEquDestab`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberRdEquDestab {
  mutation: "changeMemberRdEquDestab";
  id: string;
  newRdEquDestab: number;
}

export const parseChangeMemberRdEquDestab: NormWireReader<ChangeMemberRdEquDestab> = normWireObject<ChangeMemberRdEquDestab>({ mutation: normWireRequired(normWireLiteral("changeMemberRdEquDestab")), id: normWireRequired(normWireString), newRdEquDestab: normWireRequired(normWireNumber) });
