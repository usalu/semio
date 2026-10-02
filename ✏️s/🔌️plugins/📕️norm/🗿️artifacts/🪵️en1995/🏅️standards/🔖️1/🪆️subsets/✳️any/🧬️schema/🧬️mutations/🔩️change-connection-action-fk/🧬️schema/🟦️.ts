/** 🔩️ `change-connection-action-fk` wire twin: the leaf payload `ChangeConnectionActionFK`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeConnectionActionFK {
  connectionId: string;
  actionId: string;
  newValue: number;
}

export const parseChangeConnectionActionFK: NormWireReader<ChangeConnectionActionFK> = normWireObject<ChangeConnectionActionFK>({ connectionId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
