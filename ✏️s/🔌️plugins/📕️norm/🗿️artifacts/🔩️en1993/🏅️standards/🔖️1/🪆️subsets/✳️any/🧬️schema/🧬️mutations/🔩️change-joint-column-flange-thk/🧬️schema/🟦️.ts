/** 🔩️ `change-joint-column-flange-thk` wire twin: the leaf payload `ChangeJointColumnFlangeThk`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointColumnFlangeThk {
  id: string;
  newColumnFlangeThickness: number;
}

export const parseChangeJointColumnFlangeThk: NormWireReader<ChangeJointColumnFlangeThk> = normWireObject<ChangeJointColumnFlangeThk>({ id: normWireRequired(normWireString), newColumnFlangeThickness: normWireRequired(normWireNumber) });
