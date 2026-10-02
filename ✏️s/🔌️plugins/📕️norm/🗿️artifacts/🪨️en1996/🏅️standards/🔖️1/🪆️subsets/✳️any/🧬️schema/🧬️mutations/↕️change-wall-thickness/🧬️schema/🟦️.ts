/** ↕️ `change-wall-thickness` wire twin: the leaf payload `ChangeWallThickness`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeWallThickness {
  index: number;
  newThicknessM: number;
}

export const parseChangeWallThickness: NormWireReader<ChangeWallThickness> = normWireObject<ChangeWallThickness>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newThicknessM: normWireRequired(normWireRange(normWireNumber, {"exclusiveMinimum":0})) });
