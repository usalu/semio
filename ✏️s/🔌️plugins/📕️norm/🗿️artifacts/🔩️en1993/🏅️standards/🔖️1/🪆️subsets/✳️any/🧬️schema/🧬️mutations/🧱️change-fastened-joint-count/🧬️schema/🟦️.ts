/** 🧱️ `change-fastened-joint-count` wire twin: the leaf payload `ChangeFastenedJointCount`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFastenedJointCount {
  id: string;
  newCount: number;
}

export const parseChangeFastenedJointCount: NormWireReader<ChangeFastenedJointCount> = normWireObject<ChangeFastenedJointCount>({ id: normWireRequired(normWireString), newCount: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
