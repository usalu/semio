/** 🏛️ `change-member-punching-asw` wire twin: the leaf payload `ChangeMemberPunchingAsw`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberPunchingAsw {
  memberId: string;
  newPunchingAsw: number;
}

export const parseChangeMemberPunchingAsw: NormWireReader<ChangeMemberPunchingAsw> = normWireObject<ChangeMemberPunchingAsw>({ memberId: normWireRequired(normWireString), newPunchingAsw: normWireRequired(normWireNumber) });
