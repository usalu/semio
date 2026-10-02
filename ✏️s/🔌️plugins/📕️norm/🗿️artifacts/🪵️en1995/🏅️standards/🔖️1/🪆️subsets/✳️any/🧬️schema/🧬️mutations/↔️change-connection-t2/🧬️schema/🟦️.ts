/** ↔️ `change-connection-t2` wire twin: the leaf payload `ChangeConnectionT2`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeConnectionT2 {
  connectionId: string;
  newValue: number;
}

export const parseChangeConnectionT2: NormWireReader<ChangeConnectionT2> = normWireObject<ChangeConnectionT2>({ connectionId: normWireRequired(normWireString), newValue: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
