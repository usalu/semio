/** 🏛️ `change-member-punching-beta-method` wire twin: the leaf payload `ChangeMemberPunchingBetaMethod`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberPunchingBetaMethod {
  memberId: string;
  newPunchingBetaMethod: "Simplified" | "UnbalancedMoments";
}

export const parseChangeMemberPunchingBetaMethod: NormWireReader<ChangeMemberPunchingBetaMethod> = normWireObject<ChangeMemberPunchingBetaMethod>({ memberId: normWireRequired(normWireString), newPunchingBetaMethod: normWireRequired(normWireLiteral("Simplified", "UnbalancedMoments")) });
