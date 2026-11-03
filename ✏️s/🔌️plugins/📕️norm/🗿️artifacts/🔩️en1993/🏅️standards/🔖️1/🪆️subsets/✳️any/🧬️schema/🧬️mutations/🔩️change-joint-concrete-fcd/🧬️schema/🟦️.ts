/** 🔩️ `change-joint-concrete-fcd` wire twin: the leaf payload `ChangeJointConcreteFcd`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointConcreteFcd {
  id: string;
  newConcreteFcd: number;
}

export const parseChangeJointConcreteFcd: NormWireReader<ChangeJointConcreteFcd> = normWireObject<ChangeJointConcreteFcd>({ id: normWireRequired(normWireString), newConcreteFcd: normWireRequired(normWireNumber) });
