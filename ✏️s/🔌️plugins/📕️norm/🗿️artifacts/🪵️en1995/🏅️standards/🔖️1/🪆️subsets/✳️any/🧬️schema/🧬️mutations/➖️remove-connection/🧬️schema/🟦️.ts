/** ➖️ `remove-connection` wire twin: the leaf payload `RemoveConnection`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveConnection {
  index: number;
}

export const parseRemoveConnection: NormWireReader<RemoveConnection> = normWireObject<RemoveConnection>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
