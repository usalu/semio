/** 🧭 `change-mixed-terrain-upwind` wire twin: the leaf payload `ChangeMixedTerrainUpwind`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeMixedTerrainUpwind {
  newMixedTerrainUpwind: number;
}

export const parseChangeMixedTerrainUpwind: NormWireReader<ChangeMixedTerrainUpwind> = normWireObject<ChangeMixedTerrainUpwind>({ newMixedTerrainUpwind: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4})) });
