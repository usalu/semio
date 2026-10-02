/** ➖️ `remove-wind-faces` wire twin: the leaf payload `RemoveWindFaces`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface RemoveWindFaces {
  index: number;
}

export const parseRemoveWindFaces: NormWireReader<RemoveWindFaces> = normWireObject<RemoveWindFaces>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
