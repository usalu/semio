/** 🧱️ `change-fatigue-stress-fibre-z` wire twin: the leaf payload `ChangeFatigueStressFibreZ`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFatigueStressFibreZ {
  id: string;
  newFibreZ: number;
}

export const parseChangeFatigueStressFibreZ: NormWireReader<ChangeFatigueStressFibreZ> = normWireObject<ChangeFatigueStressFibreZ>({ id: normWireRequired(normWireString), newFibreZ: normWireRequired(normWireNumber) });
