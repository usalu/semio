/** 🧱️ `change-fastened-joint-washer-diameter` wire twin: the leaf payload `ChangeFastenedJointWasherDiameter`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFastenedJointWasherDiameter {
  id: string;
  newWasherDiameter: number;
}

export const parseChangeFastenedJointWasherDiameter: NormWireReader<ChangeFastenedJointWasherDiameter> = normWireObject<ChangeFastenedJointWasherDiameter>({ id: normWireRequired(normWireString), newWasherDiameter: normWireRequired(normWireNumber) });
