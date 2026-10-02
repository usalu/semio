/** 📏 `change-mixed-terrain-distance` wire twin: the leaf payload `ChangeMixedTerrainDistance`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeMixedTerrainDistance {
  newMixedTerrainDistance: number;
}

export const parseChangeMixedTerrainDistance: NormWireReader<ChangeMixedTerrainDistance> = normWireObject<ChangeMixedTerrainDistance>({ newMixedTerrainDistance: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
