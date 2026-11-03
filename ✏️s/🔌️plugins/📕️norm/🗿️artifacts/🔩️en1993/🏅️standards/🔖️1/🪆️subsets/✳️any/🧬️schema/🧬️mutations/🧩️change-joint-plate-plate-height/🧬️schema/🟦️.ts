/** 🧩️ `change-joint-plate-plate-height` wire twin: the leaf payload `ChangeJointPlatePlateHeight`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointPlatePlateHeight {
  id: string;
  newPlateHeight: number;
}

export const parseChangeJointPlatePlateHeight: NormWireReader<ChangeJointPlatePlateHeight> = normWireObject<ChangeJointPlatePlateHeight>({ id: normWireRequired(normWireString), newPlateHeight: normWireRequired(normWireNumber) });
