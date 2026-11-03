/** ➕️ `insert-joint` wire twin: the leaf payload `InsertJoint`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNullable, normWireObject, normWireOptional, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type SteelJoint, parseSteelJoint } from "../../../📸️snapshot/🟦️.ts";

export interface InsertJoint {
  index?: number | null;
  joint: SteelJoint;
}

export const parseInsertJoint: NormWireReader<InsertJoint> = normWireObject<InsertJoint>({ index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), joint: normWireRequired(parseSteelJoint) });
