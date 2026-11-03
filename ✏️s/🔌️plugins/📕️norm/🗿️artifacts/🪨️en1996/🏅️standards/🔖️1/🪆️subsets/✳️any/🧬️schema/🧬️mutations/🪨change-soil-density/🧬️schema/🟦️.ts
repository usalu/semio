/** 🪨 `change-soil-density` wire twin: the leaf payload `ChangeSoilDensity`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSoilDensity {
  id: string;
  newSoilDensityKgM3: number;
}

export const parseChangeSoilDensity: NormWireReader<ChangeSoilDensity> = normWireObject<ChangeSoilDensity>({ id: normWireRequired(normWireString), newSoilDensityKgM3: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":23000})) });
