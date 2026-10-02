/** ➖️ `remove-connection-action` wire twin: the leaf payload `RemoveConnectionAction`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface RemoveConnectionAction {
  connectionId: string;
  index: number;
}

export const parseRemoveConnectionAction: NormWireReader<RemoveConnectionAction> = normWireObject<RemoveConnectionAction>({ connectionId: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
