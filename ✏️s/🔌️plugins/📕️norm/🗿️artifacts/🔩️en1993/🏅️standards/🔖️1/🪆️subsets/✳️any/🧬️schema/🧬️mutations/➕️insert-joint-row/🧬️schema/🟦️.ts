/** ➕️ `insert-joint-row` wire twin: the leaf payload `InsertJointRow`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNullable, normWireObject, normWireOptional, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type JointRow, parseJointRow } from "../../../📸️snapshot/🟦️.ts";

export interface InsertJointRow {
  index?: number | null;
  jointRow: JointRow;
}

export const parseInsertJointRow: NormWireReader<InsertJointRow> = normWireObject<InsertJointRow>({ index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), jointRow: normWireRequired(parseJointRow) });
