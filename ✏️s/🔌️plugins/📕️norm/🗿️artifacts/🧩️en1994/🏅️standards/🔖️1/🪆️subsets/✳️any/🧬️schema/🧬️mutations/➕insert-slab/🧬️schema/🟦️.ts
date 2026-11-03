/** ➕ `insert-slab` wire twin: the leaf payload `InsertSlab`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type CompositeSlab, parseCompositeSlab } from "../../../📸️snapshot/🟦️.ts";

export interface InsertSlab {
  index: number;
  slab: CompositeSlab;
}

export const parseInsertSlab: NormWireReader<InsertSlab> = normWireObject<InsertSlab>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), slab: normWireRequired(parseCompositeSlab) });
