/** 🏢 `change-air-density` wire twin: the leaf payload `ChangeAirDensity`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAirDensity {
  newAirDensity: number;
}

export const parseChangeAirDensity: NormWireReader<ChangeAirDensity> = normWireObject<ChangeAirDensity>({ newAirDensity: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
