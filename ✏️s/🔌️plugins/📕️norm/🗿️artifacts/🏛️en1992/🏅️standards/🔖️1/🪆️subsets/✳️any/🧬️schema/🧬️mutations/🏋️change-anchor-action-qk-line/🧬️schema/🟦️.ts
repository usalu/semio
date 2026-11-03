/** 🏋️ `change-anchor-action-qk-line` wire twin: the leaf payload `ChangeAnchorActionQKLine`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAnchorActionQKLine {
  anchorId: string;
  actionId: string;
  newQKLine: number;
}

export const parseChangeAnchorActionQKLine: NormWireReader<ChangeAnchorActionQKLine> = normWireObject<ChangeAnchorActionQKLine>({ anchorId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newQKLine: normWireRequired(normWireNumber) });
