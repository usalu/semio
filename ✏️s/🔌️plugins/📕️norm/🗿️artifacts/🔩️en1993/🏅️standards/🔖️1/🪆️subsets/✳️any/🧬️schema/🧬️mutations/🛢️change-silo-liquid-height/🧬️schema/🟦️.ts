/** 🛢️ `change-silo-liquid-height` wire twin: the leaf payload `ChangeSiloLiquidHeight`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSiloLiquidHeight {
  id: string;
  newLiquidHeight: number;
}

export const parseChangeSiloLiquidHeight: NormWireReader<ChangeSiloLiquidHeight> = normWireObject<ChangeSiloLiquidHeight>({ id: normWireRequired(normWireString), newLiquidHeight: normWireRequired(normWireNumber) });
