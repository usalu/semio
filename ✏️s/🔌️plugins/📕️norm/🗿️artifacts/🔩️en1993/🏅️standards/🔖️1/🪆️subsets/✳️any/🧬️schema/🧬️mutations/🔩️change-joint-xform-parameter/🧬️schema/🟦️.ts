/** 🔩️ `change-joint-xform-parameter` wire twin: the leaf payload `ChangeJointXformParameter`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointXformParameter {
  id: string;
  newTransformationParameter: number;
}

export const parseChangeJointXformParameter: NormWireReader<ChangeJointXformParameter> = normWireObject<ChangeJointXformParameter>({ id: normWireRequired(normWireString), newTransformationParameter: normWireRequired(normWireNumber) });
