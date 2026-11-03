/** 🏗️ `change-member-precamber` wire twin: the leaf payload `ChangeMemberPrecamber`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberPrecamber {
  mutation: "changeMemberPrecamber";
  id: string;
  newPrecamber: number;
}

export const parseChangeMemberPrecamber: NormWireReader<ChangeMemberPrecamber> = normWireObject<ChangeMemberPrecamber>({ mutation: normWireRequired(normWireLiteral("changeMemberPrecamber")), id: normWireRequired(normWireString), newPrecamber: normWireRequired(normWireNumber) });
