/** ➕️ `insert-frame-node` wire twin: the leaf payload `InsertFrameNode`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNullable, normWireObject, normWireOptional, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type FrameNode, parseFrameNode } from "../../../📸️snapshot/🟦️.ts";

export interface InsertFrameNode {
  index?: number | null;
  frameNode: FrameNode;
}

export const parseInsertFrameNode: NormWireReader<InsertFrameNode> = normWireObject<InsertFrameNode>({ index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), frameNode: normWireRequired(parseFrameNode) });
