/** 🏗️ `change-member-rd-equ-stab` wire twin: the leaf payload `ChangeMemberRdEquStab`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberRdEquStab {
  mutation: "changeMemberRdEquStab";
  id: string;
  newRdEquStab: number;
}

export const parseChangeMemberRdEquStab: NormWireReader<ChangeMemberRdEquStab> = normWireObject<ChangeMemberRdEquStab>({ mutation: normWireRequired(normWireLiteral("changeMemberRdEquStab")), id: normWireRequired(normWireString), newRdEquStab: normWireRequired(normWireNumber) });
