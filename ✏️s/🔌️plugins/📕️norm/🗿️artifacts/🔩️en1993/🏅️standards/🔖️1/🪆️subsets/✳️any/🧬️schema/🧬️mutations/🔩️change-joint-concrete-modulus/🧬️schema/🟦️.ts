/** 🔩️ `change-joint-concrete-modulus` wire twin: the leaf payload `ChangeJointConcreteModulus`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointConcreteModulus {
  id: string;
  newConcreteModulus: number;
}

export const parseChangeJointConcreteModulus: NormWireReader<ChangeJointConcreteModulus> = normWireObject<ChangeJointConcreteModulus>({ id: normWireRequired(normWireString), newConcreteModulus: normWireRequired(normWireNumber) });
