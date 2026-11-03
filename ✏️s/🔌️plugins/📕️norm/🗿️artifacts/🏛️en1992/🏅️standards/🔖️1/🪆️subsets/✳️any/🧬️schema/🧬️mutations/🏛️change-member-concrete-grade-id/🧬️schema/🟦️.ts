/** 🏛️ `change-member-concrete-grade-id` wire twin: the leaf payload `ChangeMemberConcreteGradeId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberConcreteGradeId {
  memberId: string;
  newConcreteGradeId: string;
}

export const parseChangeMemberConcreteGradeId: NormWireReader<ChangeMemberConcreteGradeId> = normWireObject<ChangeMemberConcreteGradeId>({ memberId: normWireRequired(normWireString), newConcreteGradeId: normWireRequired(normWireString) });
