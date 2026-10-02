/** ⚖️ `change-member-mass-kg-per-m` wire twin: the leaf payload `ChangeMemberMassKgPerM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeMemberMassKgPerM {
  memberId: string;
  newValue: number;
}

export const parseChangeMemberMassKgPerM: NormWireReader<ChangeMemberMassKgPerM> = normWireObject<ChangeMemberMassKgPerM>({ memberId: normWireRequired(normWireString), newValue: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
