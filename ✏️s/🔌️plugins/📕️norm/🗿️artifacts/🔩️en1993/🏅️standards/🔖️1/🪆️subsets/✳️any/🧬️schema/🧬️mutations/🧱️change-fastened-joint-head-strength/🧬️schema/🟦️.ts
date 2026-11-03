/** 🧱️ `change-fastened-joint-head-strength` wire twin: the leaf payload `ChangeFastenedJointHeadStrength`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFastenedJointHeadStrength {
  id: string;
  newHeadStrength: number;
}

export const parseChangeFastenedJointHeadStrength: NormWireReader<ChangeFastenedJointHeadStrength> = normWireObject<ChangeFastenedJointHeadStrength>({ id: normWireRequired(normWireString), newHeadStrength: normWireRequired(normWireNumber) });
