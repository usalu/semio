/** 🔩️ `change-joint-weld-length` wire twin: the leaf payload `ChangeJointWeldLength`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointWeldLength {
  id: string;
  newWeldLength: number;
}

export const parseChangeJointWeldLength: NormWireReader<ChangeJointWeldLength> = normWireObject<ChangeJointWeldLength>({ id: normWireRequired(normWireString), newWeldLength: normWireRequired(normWireNumber) });
