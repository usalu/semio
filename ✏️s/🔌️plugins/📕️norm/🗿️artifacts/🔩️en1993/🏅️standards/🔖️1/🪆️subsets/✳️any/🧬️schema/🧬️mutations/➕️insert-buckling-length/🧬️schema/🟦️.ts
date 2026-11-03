/** ➕️ `insert-buckling-length` wire twin: the leaf payload `InsertBucklingLength`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNullable, normWireObject, normWireOptional, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type BucklingLength, parseFoundationBlock } from "../../../📸️snapshot/🟦️.ts";

export interface InsertBucklingLength {
  index?: number | null;
  bucklingLength: BucklingLength;
}

export const parseInsertBucklingLength: NormWireReader<InsertBucklingLength> = normWireObject<InsertBucklingLength>({ index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), bucklingLength: normWireRequired(parseFoundationBlock) });
