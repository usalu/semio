/** 🔩️ `change-joint-plate-thickness` wire twin: the leaf payload `ChangeJointPlateThickness`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointPlateThickness {
  id: string;
  newPlateThickness: number;
}

export const parseChangeJointPlateThickness: NormWireReader<ChangeJointPlateThickness> = normWireObject<ChangeJointPlateThickness>({ id: normWireRequired(normWireString), newPlateThickness: normWireRequired(normWireNumber) });
