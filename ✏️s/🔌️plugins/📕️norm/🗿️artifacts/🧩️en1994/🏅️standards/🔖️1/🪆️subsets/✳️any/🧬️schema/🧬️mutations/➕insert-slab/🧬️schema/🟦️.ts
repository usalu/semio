/** ➕ `insert-slab` wire twin: the leaf payload `InsertSlab`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type CompositeSlab, parseCompositeSlab } from "../../../📸️snapshot/🟦️.ts";

export interface InsertSlab {
  index: number;
  slab: CompositeSlab;
}

export const parseInsertSlab: NormWireReader<InsertSlab> = normWireObject<InsertSlab>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), slab: normWireRequired(parseCompositeSlab) });
