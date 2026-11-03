/** 🧭 `change-mixed-terrain-upwind` wire twin: the leaf payload `ChangeMixedTerrainUpwind`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMixedTerrainUpwind {
  newMixedTerrainUpwind: number;
}

export const parseChangeMixedTerrainUpwind: NormWireReader<ChangeMixedTerrainUpwind> = normWireObject<ChangeMixedTerrainUpwind>({ newMixedTerrainUpwind: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4})) });
