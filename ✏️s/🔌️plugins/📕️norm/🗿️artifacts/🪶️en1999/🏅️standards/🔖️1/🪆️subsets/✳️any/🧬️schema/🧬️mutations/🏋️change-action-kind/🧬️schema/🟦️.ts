/** 🏋️ `change-action-kind` wire twin: the leaf payload `ChangeActionKind`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type ActionKind, parseActionKind } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeActionKind {
  mutation: "changeActionKind";
  memberId: string;
  actionId: string;
  newKind: ActionKind;
}

export const parseChangeActionKind: NormWireReader<ChangeActionKind> = normWireObject<ChangeActionKind>({ mutation: normWireRequired(normWireLiteral("changeActionKind")), memberId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newKind: normWireRequired(parseActionKind) });
