/** 🏋️ `change-anchor-action-vk` wire twin: the leaf payload `ChangeAnchorActionVK`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAnchorActionVK {
  anchorId: string;
  actionId: string;
  newVK: number;
}

export const parseChangeAnchorActionVK: NormWireReader<ChangeAnchorActionVK> = normWireObject<ChangeAnchorActionVK>({ anchorId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newVK: normWireRequired(normWireNumber) });
