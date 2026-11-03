/** 🫙 `change-shell-action-kind` wire twin: the leaf payload `ChangeShellActionKind`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type ActionKind, parseActionKind } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeShellActionKind {
  mutation: "changeShellActionKind";
  shellId: string;
  actionId: string;
  newKind: ActionKind;
}

export const parseChangeShellActionKind: NormWireReader<ChangeShellActionKind> = normWireObject<ChangeShellActionKind>({ mutation: normWireRequired(normWireLiteral("changeShellActionKind")), shellId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newKind: normWireRequired(parseActionKind) });
