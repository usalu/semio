/** 🌀 `change-dynamic-mass-per-length` wire twin: the leaf payload `ChangeDynamicMassPerLength`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeDynamicMassPerLength {
  id: string;
  newMassPerLength: number;
}

export const parseChangeDynamicMassPerLength: NormWireReader<ChangeDynamicMassPerLength> = normWireObject<ChangeDynamicMassPerLength>({ id: normWireRequired(normWireString), newMassPerLength: normWireRequired(normWireNumber) });
