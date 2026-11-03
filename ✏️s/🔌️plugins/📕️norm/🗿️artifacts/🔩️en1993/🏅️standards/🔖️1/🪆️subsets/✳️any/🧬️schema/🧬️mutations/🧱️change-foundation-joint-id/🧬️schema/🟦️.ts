/** 🧱️ `change-foundation-joint-id` wire twin: the leaf payload `ChangeFoundationJointId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFoundationJointId {
  id: string;
  newJointId: string;
}

export const parseChangeFoundationJointId: NormWireReader<ChangeFoundationJointId> = normWireObject<ChangeFoundationJointId>({ id: normWireRequired(normWireString), newJointId: normWireRequired(normWireString) });
