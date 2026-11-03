/** ➕️ `insert-joint` wire twin: the leaf payload `InsertJoint`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireNullable, normWireObject, normWireOptional, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type En1998Joint, parseEn1998Joint } from "../../../📸️snapshot/🟦️.ts";

export interface InsertJoint {
  mutation: "insertJoint";
  index?: number | null;
  joint: En1998Joint;
}

export const parseInsertJoint: NormWireReader<InsertJoint> = normWireObject<InsertJoint>({ mutation: normWireRequired(normWireLiteral("insertJoint")), index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), joint: normWireRequired(parseEn1998Joint) });
