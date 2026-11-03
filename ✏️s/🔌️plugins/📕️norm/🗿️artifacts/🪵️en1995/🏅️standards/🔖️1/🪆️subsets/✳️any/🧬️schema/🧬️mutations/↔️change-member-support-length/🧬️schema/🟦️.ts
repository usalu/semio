/** ↔️ `change-member-support-length` wire twin: the leaf payload `ChangeMemberSupportLength`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberSupportLength {
  memberId: string;
  newValue: number;
}

export const parseChangeMemberSupportLength: NormWireReader<ChangeMemberSupportLength> = normWireObject<ChangeMemberSupportLength>({ memberId: normWireRequired(normWireString), newValue: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
