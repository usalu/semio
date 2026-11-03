/** 📏️ `change-member-buckling-length` wire twin: the leaf payload `ChangeMemberBucklingLength`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberBucklingLength {
  mutation: "changeMemberBucklingLength";
  memberId: string;
  axis: string;
  newLength: number;
}

export const parseChangeMemberBucklingLength: NormWireReader<ChangeMemberBucklingLength> = normWireObject<ChangeMemberBucklingLength>({ mutation: normWireRequired(normWireLiteral("changeMemberBucklingLength")), memberId: normWireRequired(normWireString), axis: normWireRequired(normWireString), newLength: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
