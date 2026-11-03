/** ❄️ `change-sheet-action-kind` wire twin: the leaf payload `ChangeSheetActionKind`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type ActionKind, parseActionKind } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeSheetActionKind {
  mutation: "changeSheetActionKind";
  coldFormedId: string;
  actionId: string;
  newKind: ActionKind;
}

export const parseChangeSheetActionKind: NormWireReader<ChangeSheetActionKind> = normWireObject<ChangeSheetActionKind>({ mutation: normWireRequired(normWireLiteral("changeSheetActionKind")), coldFormedId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newKind: normWireRequired(parseActionKind) });
