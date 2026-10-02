/** ⚖️ `change-member-mass-kg-per-m2` wire twin: the leaf payload `ChangeMemberMassKgPerM2`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeMemberMassKgPerM2 {
  memberId: string;
  newValue: number;
}

export const parseChangeMemberMassKgPerM2: NormWireReader<ChangeMemberMassKgPerM2> = normWireObject<ChangeMemberMassKgPerM2>({ memberId: normWireRequired(normWireString), newValue: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
