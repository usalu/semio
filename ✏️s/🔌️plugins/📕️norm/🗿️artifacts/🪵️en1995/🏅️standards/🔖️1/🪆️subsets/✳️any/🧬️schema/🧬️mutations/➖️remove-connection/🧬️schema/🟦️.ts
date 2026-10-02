/** ➖️ `remove-connection` wire twin: the leaf payload `RemoveConnection`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface RemoveConnection {
  index: number;
}

export const parseRemoveConnection: NormWireReader<RemoveConnection> = normWireObject<RemoveConnection>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
