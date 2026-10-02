/** 🌊️ `change-member-damping-xi` wire twin: the leaf payload `ChangeMemberDampingXi`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeMemberDampingXi {
  memberId: string;
  newValue: number;
}

export const parseChangeMemberDampingXi: NormWireReader<ChangeMemberDampingXi> = normWireObject<ChangeMemberDampingXi>({ memberId: normWireRequired(normWireString), newValue: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":1})) });
