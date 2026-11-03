/** 🔩️ `change-joint-base-lever-left` wire twin: the leaf payload `ChangeJointBaseLeverLeft`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointBaseLeverLeft {
  id: string;
  newBaseLeverLeft: number;
}

export const parseChangeJointBaseLeverLeft: NormWireReader<ChangeJointBaseLeverLeft> = normWireObject<ChangeJointBaseLeverLeft>({ id: normWireRequired(normWireString), newBaseLeverLeft: normWireRequired(normWireNumber) });
