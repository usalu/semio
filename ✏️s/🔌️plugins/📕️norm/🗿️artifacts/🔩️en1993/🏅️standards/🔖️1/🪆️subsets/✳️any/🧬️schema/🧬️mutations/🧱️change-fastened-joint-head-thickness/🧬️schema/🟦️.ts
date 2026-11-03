/** 🧱️ `change-fastened-joint-head-thickness` wire twin: the leaf payload `ChangeFastenedJointHeadThickness`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFastenedJointHeadThickness {
  id: string;
  newHeadThickness: number;
}

export const parseChangeFastenedJointHeadThickness: NormWireReader<ChangeFastenedJointHeadThickness> = normWireObject<ChangeFastenedJointHeadThickness>({ id: normWireRequired(normWireString), newHeadThickness: normWireRequired(normWireNumber) });
