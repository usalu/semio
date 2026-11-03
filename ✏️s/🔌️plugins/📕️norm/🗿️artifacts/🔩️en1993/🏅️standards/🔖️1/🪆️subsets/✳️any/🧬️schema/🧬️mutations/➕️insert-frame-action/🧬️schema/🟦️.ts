/** ➕️ `insert-frame-action` wire twin: the leaf payload `InsertFrameAction`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNullable, normWireObject, normWireOptional, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type FrameAction, parseFrameAction } from "../../../📸️snapshot/🟦️.ts";

export interface InsertFrameAction {
  index?: number | null;
  frameAction: FrameAction;
}

export const parseInsertFrameAction: NormWireReader<InsertFrameAction> = normWireObject<InsertFrameAction>({ index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), frameAction: normWireRequired(parseFrameAction) });
