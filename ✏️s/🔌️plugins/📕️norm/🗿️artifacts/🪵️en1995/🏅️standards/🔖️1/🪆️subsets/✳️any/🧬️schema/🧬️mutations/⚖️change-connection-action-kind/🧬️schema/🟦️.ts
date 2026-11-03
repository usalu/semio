/** ⚖️ `change-connection-action-kind` wire twin: the leaf payload `ChangeConnectionActionKind`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type ActionKind, parseActionKind } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeConnectionActionKind {
  connectionId: string;
  actionId: string;
  newValue: ActionKind;
}

export const parseChangeConnectionActionKind: NormWireReader<ChangeConnectionActionKind> = normWireObject<ChangeConnectionActionKind>({ connectionId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newValue: normWireRequired(parseActionKind) });
