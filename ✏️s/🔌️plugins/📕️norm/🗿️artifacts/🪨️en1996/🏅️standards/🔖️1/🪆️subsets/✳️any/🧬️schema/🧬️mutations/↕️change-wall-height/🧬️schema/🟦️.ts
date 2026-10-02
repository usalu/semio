/** ↕️ `change-wall-height` wire twin: the leaf payload `ChangeWallHeight`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeWallHeight {
  index: number;
  newHeightM: number;
}

export const parseChangeWallHeight: NormWireReader<ChangeWallHeight> = normWireObject<ChangeWallHeight>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newHeightM: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
