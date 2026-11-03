/** 🏋️ `change-anchor-action-gk-line` wire twin: the leaf payload `ChangeAnchorActionGKLine`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAnchorActionGKLine {
  anchorId: string;
  actionId: string;
  newGKLine: number;
}

export const parseChangeAnchorActionGKLine: NormWireReader<ChangeAnchorActionGKLine> = normWireObject<ChangeAnchorActionGKLine>({ anchorId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newGKLine: normWireRequired(normWireNumber) });
