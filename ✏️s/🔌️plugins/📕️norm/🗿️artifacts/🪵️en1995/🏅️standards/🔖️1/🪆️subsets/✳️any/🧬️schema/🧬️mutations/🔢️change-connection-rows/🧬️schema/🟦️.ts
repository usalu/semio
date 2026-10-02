/** 🔢️ `change-connection-rows` wire twin: the leaf payload `ChangeConnectionRows`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeConnectionRows {
  connectionId: string;
  newValue: number;
}

export const parseChangeConnectionRows: NormWireReader<ChangeConnectionRows> = normWireObject<ChangeConnectionRows>({ connectionId: normWireRequired(normWireString), newValue: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
