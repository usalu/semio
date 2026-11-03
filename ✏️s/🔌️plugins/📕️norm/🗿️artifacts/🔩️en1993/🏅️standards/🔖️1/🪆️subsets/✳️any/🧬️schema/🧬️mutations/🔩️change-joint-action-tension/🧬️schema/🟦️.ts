/** 🔩️ `change-joint-action-tension` wire twin: the leaf payload `ChangeJointActionTension`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointActionTension {
  id: string;
  elementId: string;
  newTension: number;
}

export const parseChangeJointActionTension: NormWireReader<ChangeJointActionTension> = normWireObject<ChangeJointActionTension>({ id: normWireRequired(normWireString), elementId: normWireRequired(normWireString), newTension: normWireRequired(normWireNumber) });
