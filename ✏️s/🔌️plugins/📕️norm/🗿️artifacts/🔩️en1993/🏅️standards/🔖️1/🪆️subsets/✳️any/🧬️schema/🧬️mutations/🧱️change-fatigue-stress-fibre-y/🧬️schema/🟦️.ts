/** 🧱️ `change-fatigue-stress-fibre-y` wire twin: the leaf payload `ChangeFatigueStressFibreY`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFatigueStressFibreY {
  id: string;
  newFibreY: number;
}

export const parseChangeFatigueStressFibreY: NormWireReader<ChangeFatigueStressFibreY> = normWireObject<ChangeFatigueStressFibreY>({ id: normWireRequired(normWireString), newFibreY: normWireRequired(normWireNumber) });
