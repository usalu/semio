/** 🗼️ `change-tower-mass-per-length` wire twin: the leaf payload `ChangeTowerMassPerLength`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTowerMassPerLength {
  id: string;
  newMassPerLength: number;
}

export const parseChangeTowerMassPerLength: NormWireReader<ChangeTowerMassPerLength> = normWireObject<ChangeTowerMassPerLength>({ id: normWireRequired(normWireString), newMassPerLength: normWireRequired(normWireNumber) });
