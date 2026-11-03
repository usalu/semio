/** 🔩️ `change-joint-base-plate-length` wire twin: the leaf payload `ChangeJointBasePlateLength`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointBasePlateLength {
  id: string;
  newBasePlateLength: number;
}

export const parseChangeJointBasePlateLength: NormWireReader<ChangeJointBasePlateLength> = normWireObject<ChangeJointBasePlateLength>({ id: normWireRequired(normWireString), newBasePlateLength: normWireRequired(normWireNumber) });
