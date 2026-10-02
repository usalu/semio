/** ⚖️ `change-connection-action-kind` wire twin: the leaf payload `ChangeConnectionActionKind`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeConnectionActionKind {
  connectionId: string;
  actionId: string;
  newValue: string;
}

export const parseChangeConnectionActionKind: NormWireReader<ChangeConnectionActionKind> = normWireObject<ChangeConnectionActionKind>({ connectionId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newValue: normWireRequired(normWireString) });
