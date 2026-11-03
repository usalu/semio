/** ➕️ `insert-taper` wire twin: the leaf payload `InsertTaper`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type Taper, parseTaper } from "../../../📸️snapshot/🟦️.ts";

export interface InsertTaper {
  index: number;
  taper: Taper;
}

export const parseInsertTaper: NormWireReader<InsertTaper> = normWireObject<InsertTaper>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), taper: normWireRequired(parseTaper) });
