/** 🏗️ `change-member-buckling-length-t` wire twin: the leaf payload `ChangeMemberBucklingLengthT`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberBucklingLengthT {
  mutation: "changeMemberBucklingLengthT";
  memberId: string;
  newBucklingLengthT: number;
}

export const parseChangeMemberBucklingLengthT: NormWireReader<ChangeMemberBucklingLengthT> = normWireObject<ChangeMemberBucklingLengthT>({ mutation: normWireRequired(normWireLiteral("changeMemberBucklingLengthT")), memberId: normWireRequired(normWireString), newBucklingLengthT: normWireRequired(normWireNumber) });
