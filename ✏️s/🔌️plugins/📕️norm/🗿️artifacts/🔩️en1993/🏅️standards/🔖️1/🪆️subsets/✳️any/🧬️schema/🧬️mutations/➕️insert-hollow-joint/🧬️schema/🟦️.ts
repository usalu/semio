/** ➕️ `insert-hollow-joint` wire twin: the leaf payload `InsertHollowJoint`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNullable, normWireObject, normWireOptional, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type HollowJoint, parseHollowJoint } from "../../../📸️snapshot/🟦️.ts";

export interface InsertHollowJoint {
  index?: number | null;
  hollowJoint: HollowJoint;
}

export const parseInsertHollowJoint: NormWireReader<InsertHollowJoint> = normWireObject<InsertHollowJoint>({ index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), hollowJoint: normWireRequired(parseHollowJoint) });
