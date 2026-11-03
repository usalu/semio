/** 🏋️ `change-anchor-action-point-force` wire twin: the leaf payload `ChangeAnchorActionPointForce`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAnchorActionPointForce {
  anchorId: string;
  actionId: string;
  newPointForce: number;
}

export const parseChangeAnchorActionPointForce: NormWireReader<ChangeAnchorActionPointForce> = normWireObject<ChangeAnchorActionPointForce>({ anchorId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newPointForce: normWireRequired(normWireNumber) });
