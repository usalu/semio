/** 🏋️ `change-column-action-kind` wire twin: the leaf payload `ChangeColumnActionKind`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type ActionKind, parseActionKind } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeColumnActionKind {
  columnId: string;
  id: string;
  newKind: ActionKind;
}

export const parseChangeColumnActionKind: NormWireReader<ChangeColumnActionKind> = normWireObject<ChangeColumnActionKind>({ columnId: normWireRequired(normWireString), id: normWireRequired(normWireString), newKind: normWireRequired(parseActionKind) });
