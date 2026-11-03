/** 🏋️ `change-action-kind` wire twin: the leaf payload `ChangeActionKind`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type ActionKind, parseActionKind } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeActionKind {
  memberId: string;
  actionId: string;
  newKind: ActionKind;
}

export const parseChangeActionKind: NormWireReader<ChangeActionKind> = normWireObject<ChangeActionKind>({ memberId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newKind: normWireRequired(parseActionKind) });
