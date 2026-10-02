/** ➖️ `remove-zone` wire twin: the leaf payload `RemoveZone`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface RemoveZone {
  index: number;
}

export const parseRemoveZone: NormWireReader<RemoveZone> = normWireObject<RemoveZone>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
