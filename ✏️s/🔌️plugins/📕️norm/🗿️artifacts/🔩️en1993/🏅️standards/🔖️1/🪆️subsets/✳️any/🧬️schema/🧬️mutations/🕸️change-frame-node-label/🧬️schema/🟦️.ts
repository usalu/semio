/** 🕸️ `change-frame-node-label` wire twin: the leaf payload `ChangeFrameNodeLabel`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFrameNodeLabel {
  id: string;
  newLabel: string;
}

export const parseChangeFrameNodeLabel: NormWireReader<ChangeFrameNodeLabel> = normWireObject<ChangeFrameNodeLabel>({ id: normWireRequired(normWireString), newLabel: normWireRequired(normWireString) });
