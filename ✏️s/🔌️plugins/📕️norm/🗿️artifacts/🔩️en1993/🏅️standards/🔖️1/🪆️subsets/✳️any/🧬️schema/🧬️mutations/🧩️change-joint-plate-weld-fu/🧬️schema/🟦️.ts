/** 🧩️ `change-joint-plate-weld-fu` wire twin: the leaf payload `ChangeJointPlateWeldFu`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointPlateWeldFu {
  id: string;
  newWeldFu: number;
}

export const parseChangeJointPlateWeldFu: NormWireReader<ChangeJointPlateWeldFu> = normWireObject<ChangeJointPlateWeldFu>({ id: normWireRequired(normWireString), newWeldFu: normWireRequired(normWireNumber) });
