/** ➕️ `insert-fastener` wire twin: the leaf payload `InsertFastener`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type Fastener, parseFastener } from "../../../📸️snapshot/🟦️.ts";

export interface InsertFastener {
  index: number;
  fastener: Fastener;
}

export const parseInsertFastener: NormWireReader<InsertFastener> = normWireObject<InsertFastener>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), fastener: normWireRequired(parseFastener) });
