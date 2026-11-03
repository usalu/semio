/** 🏗 `change-crane-hoist-release-fraction` wire twin: the leaf payload `ChangeCraneHoistReleaseFraction`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCraneHoistReleaseFraction {
  id: string;
  newReleaseFraction: number;
}

export const parseChangeCraneHoistReleaseFraction: NormWireReader<ChangeCraneHoistReleaseFraction> = normWireObject<ChangeCraneHoistReleaseFraction>({ id: normWireRequired(normWireString), newReleaseFraction: normWireRequired(normWireNumber) });
