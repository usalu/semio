/** 🛢️ `change-silo-liquid-density` wire twin: the leaf payload `ChangeSiloLiquidDensity`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSiloLiquidDensity {
  id: string;
  newLiquidDensity: number;
}

export const parseChangeSiloLiquidDensity: NormWireReader<ChangeSiloLiquidDensity> = normWireObject<ChangeSiloLiquidDensity>({ id: normWireRequired(normWireString), newLiquidDensity: normWireRequired(normWireNumber) });
