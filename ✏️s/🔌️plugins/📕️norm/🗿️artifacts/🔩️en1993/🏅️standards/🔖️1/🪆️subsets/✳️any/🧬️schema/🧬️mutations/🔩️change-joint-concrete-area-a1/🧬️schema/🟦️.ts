/** 🔩️ `change-joint-concrete-area-a1` wire twin: the leaf payload `ChangeJointConcreteAreaA1`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointConcreteAreaA1 {
  id: string;
  newConcreteAreaA1: number;
}

export const parseChangeJointConcreteAreaA1: NormWireReader<ChangeJointConcreteAreaA1> = normWireObject<ChangeJointConcreteAreaA1>({ id: normWireRequired(normWireString), newConcreteAreaA1: normWireRequired(normWireNumber) });
