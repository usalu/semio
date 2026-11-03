/** 🔩️ `change-joint-column-flange-thickness` wire twin: the leaf payload `ChangeJointColumnFlangeThickness`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointColumnFlangeThickness {
  id: string;
  newColumnFlangeThickness: number;
}

export const parseChangeJointColumnFlangeThickness: NormWireReader<ChangeJointColumnFlangeThickness> = normWireObject<ChangeJointColumnFlangeThickness>({ id: normWireRequired(normWireString), newColumnFlangeThickness: normWireRequired(normWireNumber) });
