/** ➕️ `insert-connection` wire twin: the leaf payload `InsertConnection`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type TimberConnection, parseTimberConnection } from "../../../📸️snapshot/🟦️.ts";

export interface InsertConnection {
  index: number;
  connection: TimberConnection;
}

export const parseInsertConnection: NormWireReader<InsertConnection> = normWireObject<InsertConnection>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), connection: normWireRequired(parseTimberConnection) });
