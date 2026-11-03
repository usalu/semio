/** 🗼️ `change-tower-air-density` wire twin: the leaf payload `ChangeTowerAirDensity`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTowerAirDensity {
  id: string;
  newAirDensity: number;
}

export const parseChangeTowerAirDensity: NormWireReader<ChangeTowerAirDensity> = normWireObject<ChangeTowerAirDensity>({ id: normWireRequired(normWireString), newAirDensity: normWireRequired(normWireNumber) });
