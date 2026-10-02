/** ↔️ `change-connection-t1` wire twin: the leaf payload `ChangeConnectionT1`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeConnectionT1 {
  connectionId: string;
  newValue: number;
}

export const parseChangeConnectionT1: NormWireReader<ChangeConnectionT1> = normWireObject<ChangeConnectionT1>({ connectionId: normWireRequired(normWireString), newValue: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
