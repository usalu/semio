/** 🏋️ `change-anchor-action-mk` wire twin: the leaf payload `ChangeAnchorActionMK`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAnchorActionMK {
  anchorId: string;
  actionId: string;
  newMK: number;
}

export const parseChangeAnchorActionMK: NormWireReader<ChangeAnchorActionMK> = normWireObject<ChangeAnchorActionMK>({ anchorId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newMK: normWireRequired(normWireNumber) });
