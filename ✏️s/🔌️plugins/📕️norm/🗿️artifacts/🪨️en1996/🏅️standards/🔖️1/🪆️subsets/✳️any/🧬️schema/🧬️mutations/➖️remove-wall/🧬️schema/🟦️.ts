/** ➖️ `remove-wall` wire twin: the leaf payload `RemoveWall`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface RemoveWall {
  index: number;
}

export const parseRemoveWall: NormWireReader<RemoveWall> = normWireObject<RemoveWall>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
