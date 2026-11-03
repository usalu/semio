/** 🔩️ `change-joint-end-distance` wire twin: the leaf payload `ChangeJointEndDistance`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointEndDistance {
  id: string;
  newEndDistance: number;
}

export const parseChangeJointEndDistance: NormWireReader<ChangeJointEndDistance> = normWireObject<ChangeJointEndDistance>({ id: normWireRequired(normWireString), newEndDistance: normWireRequired(normWireNumber) });
