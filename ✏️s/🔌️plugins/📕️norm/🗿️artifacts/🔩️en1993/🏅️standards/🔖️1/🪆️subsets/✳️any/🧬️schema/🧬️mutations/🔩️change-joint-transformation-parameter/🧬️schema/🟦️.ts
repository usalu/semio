/** 🔩️ `change-joint-transformation-parameter` wire twin: the leaf payload `ChangeJointTransformationParameter`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointTransformationParameter {
  id: string;
  newTransformationParameter: number;
}

export const parseChangeJointTransformationParameter: NormWireReader<ChangeJointTransformationParameter> = normWireObject<ChangeJointTransformationParameter>({ id: normWireRequired(normWireString), newTransformationParameter: normWireRequired(normWireNumber) });
