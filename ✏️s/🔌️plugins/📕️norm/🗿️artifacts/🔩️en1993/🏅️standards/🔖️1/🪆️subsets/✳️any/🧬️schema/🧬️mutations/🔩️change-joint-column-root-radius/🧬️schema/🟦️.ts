/** 🔩️ `change-joint-column-root-radius` wire twin: the leaf payload `ChangeJointColumnRootRadius`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointColumnRootRadius {
  id: string;
  newColumnRootRadius: number;
}

export const parseChangeJointColumnRootRadius: NormWireReader<ChangeJointColumnRootRadius> = normWireObject<ChangeJointColumnRootRadius>({ id: normWireRequired(normWireString), newColumnRootRadius: normWireRequired(normWireNumber) });
