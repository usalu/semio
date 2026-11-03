/** 🧨 `change-explosion-vent-pressure` wire twin: the leaf payload `ChangeExplosionVentPressure`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeExplosionVentPressure {
  id: string;
  newVentPressure: number;
}

export const parseChangeExplosionVentPressure: NormWireReader<ChangeExplosionVentPressure> = normWireObject<ChangeExplosionVentPressure>({ id: normWireRequired(normWireString), newVentPressure: normWireRequired(normWireNumber) });
