/** 🔩️ `change-joint-action-shear` wire twin: the leaf payload `ChangeJointActionShear`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointActionShear {
  id: string;
  elementId: string;
  newShear: number;
}

export const parseChangeJointActionShear: NormWireReader<ChangeJointActionShear> = normWireObject<ChangeJointActionShear>({ id: normWireRequired(normWireString), elementId: normWireRequired(normWireString), newShear: normWireRequired(normWireNumber) });
