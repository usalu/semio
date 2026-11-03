/** 🧱️ `change-fastened-joint-end-distance` wire twin: the leaf payload `ChangeFastenedJointEndDistance`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFastenedJointEndDistance {
  id: string;
  newEndDistance: number;
}

export const parseChangeFastenedJointEndDistance: NormWireReader<ChangeFastenedJointEndDistance> = normWireObject<ChangeFastenedJointEndDistance>({ id: normWireRequired(normWireString), newEndDistance: normWireRequired(normWireNumber) });
