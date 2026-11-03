/** ↔️ `change-connection-t1` wire twin: the leaf payload `ChangeConnectionT1`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeConnectionT1 {
  connectionId: string;
  newValue: number;
}

export const parseChangeConnectionT1: NormWireReader<ChangeConnectionT1> = normWireObject<ChangeConnectionT1>({ connectionId: normWireRequired(normWireString), newValue: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
