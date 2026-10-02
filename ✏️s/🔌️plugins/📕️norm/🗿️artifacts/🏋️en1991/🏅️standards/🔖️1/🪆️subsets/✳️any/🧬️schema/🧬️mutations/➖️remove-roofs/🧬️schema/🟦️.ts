/** ➖️ `remove-roofs` wire twin: the leaf payload `RemoveRoofs`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface RemoveRoofs {
  index: number;
}

export const parseRemoveRoofs: NormWireReader<RemoveRoofs> = normWireObject<RemoveRoofs>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
