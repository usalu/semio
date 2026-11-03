/** 🧱️ `change-frame-action-target-id` wire twin: the leaf payload `ChangeFrameActionTargetId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFrameActionTargetId {
  id: string;
  newTargetId: string;
}

export const parseChangeFrameActionTargetId: NormWireReader<ChangeFrameActionTargetId> = normWireObject<ChangeFrameActionTargetId>({ id: normWireRequired(normWireString), newTargetId: normWireRequired(normWireString) });
