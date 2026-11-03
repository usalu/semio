/** 🔢️ `change-connection-rows` wire twin: the leaf payload `ChangeConnectionRows`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeConnectionRows {
  connectionId: string;
  newValue: number;
}

export const parseChangeConnectionRows: NormWireReader<ChangeConnectionRows> = normWireObject<ChangeConnectionRows>({ connectionId: normWireRequired(normWireString), newValue: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
