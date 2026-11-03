/** 🧱️ `change-fastened-joint-base-thickness` wire twin: the leaf payload `ChangeFastenedJointBaseThickness`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFastenedJointBaseThickness {
  id: string;
  newBaseThickness: number;
}

export const parseChangeFastenedJointBaseThickness: NormWireReader<ChangeFastenedJointBaseThickness> = normWireObject<ChangeFastenedJointBaseThickness>({ id: normWireRequired(normWireString), newBaseThickness: normWireRequired(normWireNumber) });
