/** ⚙ `insert-snow-guard` wire twin: the leaf payload `InsertSnowGuard`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type SnowGuard, parseSnowGuard } from "../../../📸️snapshot/🟦️.ts";

export interface InsertSnowGuard {
  index: number;
  item: SnowGuard;
}

export const parseInsertSnowGuard: NormWireReader<InsertSnowGuard> = normWireObject<InsertSnowGuard>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), item: normWireRequired(parseSnowGuard) });
