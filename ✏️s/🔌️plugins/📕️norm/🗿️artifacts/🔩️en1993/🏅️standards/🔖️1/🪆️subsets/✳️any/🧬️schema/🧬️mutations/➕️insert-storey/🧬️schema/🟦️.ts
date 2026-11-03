/** ➕️ `insert-storey` wire twin: the leaf payload `InsertStorey`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNullable, normWireObject, normWireOptional, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type Storey, parseStorey } from "../../../📸️snapshot/🟦️.ts";

export interface InsertStorey {
  index?: number | null;
  storey: Storey;
}

export const parseInsertStorey: NormWireReader<InsertStorey> = normWireObject<InsertStorey>({ index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), storey: normWireRequired(parseStorey) });
