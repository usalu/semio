/** 🏷️ `change-connection-label-de` wire twin: the leaf payload `ChangeConnectionLabelDe`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeConnectionLabelDe {
  connectionId: string;
  newValue: string;
}

export const parseChangeConnectionLabelDe: NormWireReader<ChangeConnectionLabelDe> = normWireObject<ChangeConnectionLabelDe>({ connectionId: normWireRequired(normWireString), newValue: normWireRequired(normWireString) });
