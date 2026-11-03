/** 🔩️ `change-joint-weld-fu` wire twin: the leaf payload `ChangeJointWeldFu`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointWeldFu {
  id: string;
  newWeldFu: number;
}

export const parseChangeJointWeldFu: NormWireReader<ChangeJointWeldFu> = normWireObject<ChangeJointWeldFu>({ id: normWireRequired(normWireString), newWeldFu: normWireRequired(normWireNumber) });
