/** 🔗️ `change-conn-action-kind` wire twin: the leaf payload `ChangeConnActionKind`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type ActionKind, parseActionKind } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeConnActionKind {
  mutation: "changeConnActionKind";
  connectionId: string;
  actionId: string;
  newKind: ActionKind;
}

export const parseChangeConnActionKind: NormWireReader<ChangeConnActionKind> = normWireObject<ChangeConnActionKind>({ mutation: normWireRequired(normWireLiteral("changeConnActionKind")), connectionId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newKind: normWireRequired(parseActionKind) });
