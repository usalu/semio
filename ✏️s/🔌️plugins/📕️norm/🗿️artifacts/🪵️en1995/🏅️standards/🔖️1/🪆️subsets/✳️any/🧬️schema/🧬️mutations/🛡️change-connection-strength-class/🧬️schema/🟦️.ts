/** 🛡️ `change-connection-strength-class` wire twin: the leaf payload `ChangeConnectionStrengthClass`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeConnectionStrengthClass {
  connectionId: string;
  newValue: string;
}

export const parseChangeConnectionStrengthClass: NormWireReader<ChangeConnectionStrengthClass> = normWireObject<ChangeConnectionStrengthClass>({ connectionId: normWireRequired(normWireString), newValue: normWireRequired(normWireString) });
