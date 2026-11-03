/** 🏛️ `change-member-rebar-grade-id` wire twin: the leaf payload `ChangeMemberRebarGradeId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberRebarGradeId {
  memberId: string;
  newReinforcementGradeId: string;
}

export const parseChangeMemberRebarGradeId: NormWireReader<ChangeMemberRebarGradeId> = normWireObject<ChangeMemberRebarGradeId>({ memberId: normWireRequired(normWireString), newReinforcementGradeId: normWireRequired(normWireString) });
