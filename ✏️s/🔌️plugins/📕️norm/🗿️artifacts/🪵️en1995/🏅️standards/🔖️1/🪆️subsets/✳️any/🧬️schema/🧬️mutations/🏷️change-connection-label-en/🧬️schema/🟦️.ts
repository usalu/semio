/** 🏷️ `change-connection-label-en` wire twin: the leaf payload `ChangeConnectionLabelEn`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeConnectionLabelEn {
  connectionId: string;
  newValue: string;
}

export const parseChangeConnectionLabelEn: NormWireReader<ChangeConnectionLabelEn> = normWireObject<ChangeConnectionLabelEn>({ connectionId: normWireRequired(normWireString), newValue: normWireRequired(normWireString) });
