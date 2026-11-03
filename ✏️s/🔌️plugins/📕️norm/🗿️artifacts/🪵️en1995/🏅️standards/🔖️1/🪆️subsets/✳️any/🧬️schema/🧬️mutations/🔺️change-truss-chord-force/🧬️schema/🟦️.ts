/** 🔺️ `change-truss-chord-force` wire twin: the leaf payload `ChangeTrussChordForce`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTrussChordForce {
  trussId: string;
  newValue: number;
}

export const parseChangeTrussChordForce: NormWireReader<ChangeTrussChordForce> = normWireObject<ChangeTrussChordForce>({ trussId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
