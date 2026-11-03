/** 🧱️ `change-fastened-joint-base-strength` wire twin: the leaf payload `ChangeFastenedJointBaseStrength`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFastenedJointBaseStrength {
  id: string;
  newBaseStrength: number;
}

export const parseChangeFastenedJointBaseStrength: NormWireReader<ChangeFastenedJointBaseStrength> = normWireObject<ChangeFastenedJointBaseStrength>({ id: normWireRequired(normWireString), newBaseStrength: normWireRequired(normWireNumber) });
