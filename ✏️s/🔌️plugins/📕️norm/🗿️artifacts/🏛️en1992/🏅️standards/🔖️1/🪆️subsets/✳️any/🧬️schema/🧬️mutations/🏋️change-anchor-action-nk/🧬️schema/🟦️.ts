/** 🏋️ `change-anchor-action-nk` wire twin: the leaf payload `ChangeAnchorActionNK`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAnchorActionNK {
  anchorId: string;
  actionId: string;
  newNK: number;
}

export const parseChangeAnchorActionNK: NormWireReader<ChangeAnchorActionNK> = normWireObject<ChangeAnchorActionNK>({ anchorId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newNK: normWireRequired(normWireNumber) });
