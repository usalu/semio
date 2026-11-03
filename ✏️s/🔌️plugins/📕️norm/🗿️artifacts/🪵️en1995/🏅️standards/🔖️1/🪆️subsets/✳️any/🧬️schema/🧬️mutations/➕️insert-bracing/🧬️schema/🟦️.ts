/** ➕️ `insert-bracing` wire twin: the leaf payload `InsertBracing`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type Bracing, parseBracing } from "../../../📸️snapshot/🟦️.ts";

export interface InsertBracing {
  index: number;
  bracing: Bracing;
}

export const parseInsertBracing: NormWireReader<InsertBracing> = normWireObject<InsertBracing>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), bracing: normWireRequired(parseBracing) });
