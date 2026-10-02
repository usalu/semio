/** ➖️ `remove-material` wire twin: the leaf payload `RemoveMaterial`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface RemoveMaterial {
  index: number;
}

export const parseRemoveMaterial: NormWireReader<RemoveMaterial> = normWireObject<RemoveMaterial>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
