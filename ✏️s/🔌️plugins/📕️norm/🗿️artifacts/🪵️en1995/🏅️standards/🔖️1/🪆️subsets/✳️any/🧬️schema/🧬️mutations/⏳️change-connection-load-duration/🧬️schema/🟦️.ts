/** ⏳️ `change-connection-load-duration` wire twin: the leaf payload `ChangeConnectionLoadDuration`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeConnectionLoadDuration {
  connectionId: string;
  actionId: string;
  newValue: string;
}

export const parseChangeConnectionLoadDuration: NormWireReader<ChangeConnectionLoadDuration> = normWireObject<ChangeConnectionLoadDuration>({ connectionId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newValue: normWireRequired(normWireString) });
