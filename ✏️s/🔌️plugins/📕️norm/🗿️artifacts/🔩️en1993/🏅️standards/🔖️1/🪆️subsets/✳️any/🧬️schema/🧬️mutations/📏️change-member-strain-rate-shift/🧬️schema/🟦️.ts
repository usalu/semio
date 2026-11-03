/** 📏️ `change-member-strain-rate-shift` wire twin: the leaf payload `ChangeMemberStrainRateShift`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberStrainRateShift {
  id: string;
  newStrainRateShift: number;
}

export const parseChangeMemberStrainRateShift: NormWireReader<ChangeMemberStrainRateShift> = normWireObject<ChangeMemberStrainRateShift>({ id: normWireRequired(normWireString), newStrainRateShift: normWireRequired(normWireNumber) });
