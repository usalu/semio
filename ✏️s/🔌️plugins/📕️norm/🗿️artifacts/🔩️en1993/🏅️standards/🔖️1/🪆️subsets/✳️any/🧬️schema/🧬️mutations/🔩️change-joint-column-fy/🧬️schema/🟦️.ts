/** 🔩️ `change-joint-column-fy` wire twin: the leaf payload `ChangeJointColumnFy`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointColumnFy {
  id: string;
  newColumnFy: number;
}

export const parseChangeJointColumnFy: NormWireReader<ChangeJointColumnFy> = normWireObject<ChangeJointColumnFy>({ id: normWireRequired(normWireString), newColumnFy: normWireRequired(normWireNumber) });
