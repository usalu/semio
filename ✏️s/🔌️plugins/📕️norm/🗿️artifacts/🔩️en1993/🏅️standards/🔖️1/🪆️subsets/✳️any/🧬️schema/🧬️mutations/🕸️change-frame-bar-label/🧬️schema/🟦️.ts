/** 🕸️ `change-frame-bar-label` wire twin: the leaf payload `ChangeFrameBarLabel`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFrameBarLabel {
  id: string;
  newLabel: string;
}

export const parseChangeFrameBarLabel: NormWireReader<ChangeFrameBarLabel> = normWireObject<ChangeFrameBarLabel>({ id: normWireRequired(normWireString), newLabel: normWireRequired(normWireString) });
