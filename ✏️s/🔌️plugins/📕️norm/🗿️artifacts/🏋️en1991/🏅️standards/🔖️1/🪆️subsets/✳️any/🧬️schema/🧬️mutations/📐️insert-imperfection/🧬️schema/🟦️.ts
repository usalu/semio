/** 📐 `insert-imperfection` wire twin: the leaf payload `InsertImperfection`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type ExecutionImperfection, parseExecutionImperfection } from "../../../📸️snapshot/🟦️.ts";

export interface InsertImperfection {
  index: number;
  item: ExecutionImperfection;
}

export const parseInsertImperfection: NormWireReader<InsertImperfection> = normWireObject<InsertImperfection>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), item: normWireRequired(parseExecutionImperfection) });
