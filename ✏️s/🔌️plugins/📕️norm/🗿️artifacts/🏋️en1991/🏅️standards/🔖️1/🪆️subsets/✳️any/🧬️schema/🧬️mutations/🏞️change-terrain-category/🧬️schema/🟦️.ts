/** 🏞️ `change-terrain-category` wire twin: the leaf payload `ChangeTerrainCategory`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTerrainCategory {
  newTerrainCategory: number;
}

export const parseChangeTerrainCategory: NormWireReader<ChangeTerrainCategory> = normWireObject<ChangeTerrainCategory>({ newTerrainCategory: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4})) });
