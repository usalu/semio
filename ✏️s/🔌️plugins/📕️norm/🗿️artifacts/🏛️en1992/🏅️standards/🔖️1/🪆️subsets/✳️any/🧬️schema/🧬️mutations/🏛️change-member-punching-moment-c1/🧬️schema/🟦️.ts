/** 🏛️ `change-member-punching-moment-c1` wire twin: the leaf payload `ChangeMemberPunchingMomentC1`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberPunchingMomentC1 {
  memberId: string;
  newPunchingMomentC1: number;
}

export const parseChangeMemberPunchingMomentC1: NormWireReader<ChangeMemberPunchingMomentC1> = normWireObject<ChangeMemberPunchingMomentC1>({ memberId: normWireRequired(normWireString), newPunchingMomentC1: normWireRequired(normWireNumber) });
