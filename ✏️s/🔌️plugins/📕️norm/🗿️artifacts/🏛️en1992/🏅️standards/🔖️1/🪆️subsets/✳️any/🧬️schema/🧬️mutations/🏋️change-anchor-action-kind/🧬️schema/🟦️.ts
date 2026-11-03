/** 🏋️ `change-anchor-action-kind` wire twin: the leaf payload `ChangeAnchorActionKind`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type ActionKind, parseActionKind } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeAnchorActionKind {
  anchorId: string;
  actionId: string;
  newKind: ActionKind;
}

export const parseChangeAnchorActionKind: NormWireReader<ChangeAnchorActionKind> = normWireObject<ChangeAnchorActionKind>({ anchorId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newKind: normWireRequired(parseActionKind) });
