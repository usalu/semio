/** ➕️ `insert-fastened-joint` wire twin: the leaf payload `InsertFastenedJoint`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNullable, normWireObject, normWireOptional, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type FastenedJoint, parseFrameMemberBasis } from "../../../📸️snapshot/🟦️.ts";

export interface InsertFastenedJoint {
  index?: number | null;
  fastenedJoint: FastenedJoint;
}

export const parseInsertFastenedJoint: NormWireReader<InsertFastenedJoint> = normWireObject<InsertFastenedJoint>({ index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), fastenedJoint: normWireRequired(parseFrameMemberBasis) });
