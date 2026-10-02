/** ⛔️ `remove-column` wire twin: the leaf payload `RemoveColumn`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface RemoveColumn {
  index: number;
}

export const parseRemoveColumn: NormWireReader<RemoveColumn> = normWireObject<RemoveColumn>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
