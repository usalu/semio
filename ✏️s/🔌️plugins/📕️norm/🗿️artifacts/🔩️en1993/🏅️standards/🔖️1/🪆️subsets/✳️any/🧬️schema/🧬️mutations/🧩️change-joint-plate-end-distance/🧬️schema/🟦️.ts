/** 🧩️ `change-joint-plate-end-distance` wire twin: the leaf payload `ChangeJointPlateEndDistance`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointPlateEndDistance {
  id: string;
  newEndDistance: number;
}

export const parseChangeJointPlateEndDistance: NormWireReader<ChangeJointPlateEndDistance> = normWireObject<ChangeJointPlateEndDistance>({ id: normWireRequired(normWireString), newEndDistance: normWireRequired(normWireNumber) });
