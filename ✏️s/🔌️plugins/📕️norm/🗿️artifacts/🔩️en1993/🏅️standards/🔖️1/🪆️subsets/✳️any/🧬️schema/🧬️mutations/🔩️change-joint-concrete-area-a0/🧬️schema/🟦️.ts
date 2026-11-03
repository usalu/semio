/** 🔩️ `change-joint-concrete-area-a0` wire twin: the leaf payload `ChangeJointConcreteAreaA0`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointConcreteAreaA0 {
  id: string;
  newConcreteAreaA0: number;
}

export const parseChangeJointConcreteAreaA0: NormWireReader<ChangeJointConcreteAreaA0> = normWireObject<ChangeJointConcreteAreaA0>({ id: normWireRequired(normWireString), newConcreteAreaA0: normWireRequired(normWireNumber) });
