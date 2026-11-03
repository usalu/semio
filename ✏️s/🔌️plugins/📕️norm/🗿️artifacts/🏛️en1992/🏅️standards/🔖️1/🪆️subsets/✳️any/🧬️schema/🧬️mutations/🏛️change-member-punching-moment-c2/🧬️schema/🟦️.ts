/** 🏛️ `change-member-punching-moment-c2` wire twin: the leaf payload `ChangeMemberPunchingMomentC2`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberPunchingMomentC2 {
  memberId: string;
  newPunchingMomentC2: number;
}

export const parseChangeMemberPunchingMomentC2: NormWireReader<ChangeMemberPunchingMomentC2> = normWireObject<ChangeMemberPunchingMomentC2>({ memberId: normWireRequired(normWireString), newPunchingMomentC2: normWireRequired(normWireNumber) });
