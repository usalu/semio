/** 🔩️ `change-joint-base-lever-right` wire twin: the leaf payload `ChangeJointBaseLeverRight`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointBaseLeverRight {
  id: string;
  newBaseLeverRight: number;
}

export const parseChangeJointBaseLeverRight: NormWireReader<ChangeJointBaseLeverRight> = normWireObject<ChangeJointBaseLeverRight>({ id: normWireRequired(normWireString), newBaseLeverRight: normWireRequired(normWireNumber) });
