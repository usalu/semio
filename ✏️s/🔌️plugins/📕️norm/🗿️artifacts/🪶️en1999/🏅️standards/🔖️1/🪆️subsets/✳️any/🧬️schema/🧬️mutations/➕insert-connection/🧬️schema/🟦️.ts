/** 🗂️ `insert-connection` wire twin: the leaf payload `InsertConnection`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type AluminiumConnection, parseAluminiumConnection } from "../../../📸️snapshot/🟦️.ts";

export interface InsertConnection {
  mutation: "insertConnection";
  index: number;
  connection: AluminiumConnection;
}

export const parseInsertConnection: NormWireReader<InsertConnection> = normWireObject<InsertConnection>({ mutation: normWireRequired(normWireLiteral("insertConnection")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), connection: normWireRequired(parseAluminiumConnection) });
