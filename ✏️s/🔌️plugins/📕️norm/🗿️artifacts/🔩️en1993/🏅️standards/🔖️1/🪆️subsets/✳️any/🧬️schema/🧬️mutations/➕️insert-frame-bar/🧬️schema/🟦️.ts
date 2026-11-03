/** ➕️ `insert-frame-bar` wire twin: the leaf payload `InsertFrameBar`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNullable, normWireObject, normWireOptional, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type FrameBar, parseFrameBar } from "../../../📸️snapshot/🟦️.ts";

export interface InsertFrameBar {
  index?: number | null;
  frameBar: FrameBar;
}

export const parseInsertFrameBar: NormWireReader<InsertFrameBar> = normWireObject<InsertFrameBar>({ index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), frameBar: normWireRequired(parseFrameBar) });
