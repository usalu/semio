/** 🏋️ `change-anchor-action-tk` wire twin: the leaf payload `ChangeAnchorActionTK`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAnchorActionTK {
  anchorId: string;
  actionId: string;
  newTK: number;
}

export const parseChangeAnchorActionTK: NormWireReader<ChangeAnchorActionTK> = normWireObject<ChangeAnchorActionTK>({ anchorId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newTK: normWireRequired(normWireNumber) });
