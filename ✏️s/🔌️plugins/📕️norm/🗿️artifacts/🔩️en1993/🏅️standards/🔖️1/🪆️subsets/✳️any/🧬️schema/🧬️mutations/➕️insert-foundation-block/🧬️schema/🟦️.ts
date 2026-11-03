/** ➕️ `insert-foundation-block` wire twin: the leaf payload `InsertFoundationBlock`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNullable, normWireObject, normWireOptional, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type FoundationBlock, parseFoundationBlock } from "../../../📸️snapshot/🟦️.ts";

export interface InsertFoundationBlock {
  index?: number | null;
  foundationBlock: FoundationBlock;
}

export const parseInsertFoundationBlock: NormWireReader<InsertFoundationBlock> = normWireObject<InsertFoundationBlock>({ index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), foundationBlock: normWireRequired(parseFoundationBlock) });
