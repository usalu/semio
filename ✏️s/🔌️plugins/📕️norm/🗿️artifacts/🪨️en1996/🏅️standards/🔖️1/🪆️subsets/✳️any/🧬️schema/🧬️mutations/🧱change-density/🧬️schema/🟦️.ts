/** 🧱 `change-density` wire twin: the leaf payload `ChangeDensity`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeDensity {
  id: string;
  newDensityKgM3: number;
}

export const parseChangeDensity: NormWireReader<ChangeDensity> = normWireObject<ChangeDensity>({ id: normWireRequired(normWireString), newDensityKgM3: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":23000})) });
