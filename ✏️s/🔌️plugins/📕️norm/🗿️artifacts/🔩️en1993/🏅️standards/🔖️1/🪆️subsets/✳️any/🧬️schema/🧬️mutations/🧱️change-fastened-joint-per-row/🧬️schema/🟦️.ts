/** 🧱️ `change-fastened-joint-per-row` wire twin: the leaf payload `ChangeFastenedJointPerRow`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFastenedJointPerRow {
  id: string;
  newPerRow: number;
}

export const parseChangeFastenedJointPerRow: NormWireReader<ChangeFastenedJointPerRow> = normWireObject<ChangeFastenedJointPerRow>({ id: normWireRequired(normWireString), newPerRow: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
