/** ➕️ `insert-connection` wire twin: the leaf payload `InsertConnection`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { parseTimberConnection, type TimberConnection } from "../../../📸️snapshot/🟦️.ts";

export interface InsertConnection {
  index: number;
  connection: TimberConnection;
}

export const parseInsertConnection: NormWireReader<InsertConnection> = normWireObject<InsertConnection>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), connection: normWireRequired(parseTimberConnection) });
