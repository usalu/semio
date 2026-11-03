/** 🏋️ `change-anchor-action-vk-punch` wire twin: the leaf payload `ChangeAnchorActionVKPunch`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAnchorActionVKPunch {
  anchorId: string;
  actionId: string;
  newVKPunch: number;
}

export const parseChangeAnchorActionVKPunch: NormWireReader<ChangeAnchorActionVKPunch> = normWireObject<ChangeAnchorActionVKPunch>({ anchorId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newVKPunch: normWireRequired(normWireNumber) });
