/** ⚖️ `change-member-mass-kg-per-m2` wire twin: the leaf payload `ChangeMemberMassKgPerM2`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberMassKgPerM2 {
  memberId: string;
  newValue: number;
}

export const parseChangeMemberMassKgPerM2: NormWireReader<ChangeMemberMassKgPerM2> = normWireObject<ChangeMemberMassKgPerM2>({ memberId: normWireRequired(normWireString), newValue: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
