/** ➕️ `insert-frame-member-basis` wire twin: the leaf payload `InsertFrameMemberBasis`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNullable, normWireObject, normWireOptional, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type FrameMemberBasis, parseFrameMemberBasis } from "../../../📸️snapshot/🟦️.ts";

export interface InsertFrameMemberBasis {
  index?: number | null;
  frameMemberBasis: FrameMemberBasis;
}

export const parseInsertFrameMemberBasis: NormWireReader<InsertFrameMemberBasis> = normWireObject<InsertFrameMemberBasis>({ index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), frameMemberBasis: normWireRequired(parseFrameMemberBasis) });
