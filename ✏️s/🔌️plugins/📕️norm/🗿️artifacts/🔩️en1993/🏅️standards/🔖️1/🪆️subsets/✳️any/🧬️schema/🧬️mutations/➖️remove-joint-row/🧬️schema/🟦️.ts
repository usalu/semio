/** ➖️ `remove-joint-row` wire twin: the leaf payload `RemoveJointRow`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveJointRow {
  index: number;
}

export const parseRemoveJointRow: NormWireReader<RemoveJointRow> = normWireObject<RemoveJointRow>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
