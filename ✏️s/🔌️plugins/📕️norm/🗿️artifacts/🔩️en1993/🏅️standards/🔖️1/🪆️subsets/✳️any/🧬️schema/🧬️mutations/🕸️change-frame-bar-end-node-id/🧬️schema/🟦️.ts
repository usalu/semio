/** 🕸️ `change-frame-bar-end-node-id` wire twin: the leaf payload `ChangeFrameBarEndNodeId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFrameBarEndNodeId {
  id: string;
  newEndNodeId: string;
}

export const parseChangeFrameBarEndNodeId: NormWireReader<ChangeFrameBarEndNodeId> = normWireObject<ChangeFrameBarEndNodeId>({ id: normWireRequired(normWireString), newEndNodeId: normWireRequired(normWireString) });
