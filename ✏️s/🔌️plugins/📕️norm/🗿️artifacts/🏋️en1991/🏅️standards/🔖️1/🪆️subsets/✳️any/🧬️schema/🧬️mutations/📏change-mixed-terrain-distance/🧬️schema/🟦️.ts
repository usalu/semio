/** 📏 `change-mixed-terrain-distance` wire twin: the leaf payload `ChangeMixedTerrainDistance`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMixedTerrainDistance {
  newMixedTerrainDistance: number;
}

export const parseChangeMixedTerrainDistance: NormWireReader<ChangeMixedTerrainDistance> = normWireObject<ChangeMixedTerrainDistance>({ newMixedTerrainDistance: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
