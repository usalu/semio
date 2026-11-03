/** ➕️ `insert-joint-plate` wire twin: the leaf payload `InsertJointPlate`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNullable, normWireObject, normWireOptional, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type JointPlate, parseJointPlate } from "../../../📸️snapshot/🟦️.ts";

export interface InsertJointPlate {
  index?: number | null;
  jointPlate: JointPlate;
}

export const parseInsertJointPlate: NormWireReader<InsertJointPlate> = normWireObject<InsertJointPlate>({ index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), jointPlate: normWireRequired(parseJointPlate) });
