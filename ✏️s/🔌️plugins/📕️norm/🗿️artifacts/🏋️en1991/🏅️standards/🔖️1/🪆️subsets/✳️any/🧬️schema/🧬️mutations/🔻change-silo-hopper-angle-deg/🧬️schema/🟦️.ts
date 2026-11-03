/** 🔻 `change-silo-hopper-angle-deg` wire twin: the leaf payload `ChangeSiloHopperAngleDeg`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSiloHopperAngleDeg {
  newSiloHopperAngleDeg: number;
}

export const parseChangeSiloHopperAngleDeg: NormWireReader<ChangeSiloHopperAngleDeg> = normWireObject<ChangeSiloHopperAngleDeg>({ newSiloHopperAngleDeg: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":90})) });
