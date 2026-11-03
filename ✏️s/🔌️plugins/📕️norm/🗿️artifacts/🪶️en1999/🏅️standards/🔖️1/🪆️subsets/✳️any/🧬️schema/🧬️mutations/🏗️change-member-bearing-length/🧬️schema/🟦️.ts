/** 🏗️ `change-member-bearing-length` wire twin: the leaf payload `ChangeMemberBearingLength`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberBearingLength {
  mutation: "changeMemberBearingLength";
  memberId: string;
  newBearingLength: number;
}

export const parseChangeMemberBearingLength: NormWireReader<ChangeMemberBearingLength> = normWireObject<ChangeMemberBearingLength>({ mutation: normWireRequired(normWireLiteral("changeMemberBearingLength")), memberId: normWireRequired(normWireString), newBearingLength: normWireRequired(normWireNumber) });
