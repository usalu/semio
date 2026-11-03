/** 🧩️ `change-joint-plate-plate-count` wire twin: the leaf payload `ChangeJointPlatePlateCount`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointPlatePlateCount {
  id: string;
  newPlateCount: number;
}

export const parseChangeJointPlatePlateCount: NormWireReader<ChangeJointPlatePlateCount> = normWireObject<ChangeJointPlatePlateCount>({ id: normWireRequired(normWireString), newPlateCount: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
