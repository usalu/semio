/** 🕸️ `change-frame-bar-start-node-id` wire twin: the leaf payload `ChangeFrameBarStartNodeId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFrameBarStartNodeId {
  id: string;
  newStartNodeId: string;
}

export const parseChangeFrameBarStartNodeId: NormWireReader<ChangeFrameBarStartNodeId> = normWireObject<ChangeFrameBarStartNodeId>({ id: normWireRequired(normWireString), newStartNodeId: normWireRequired(normWireString) });
