/** ⚖️ `change-member-action-kind` wire twin: the leaf payload `ChangeMemberActionKind`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type ActionKind, parseActionKind } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeMemberActionKind {
  memberId: string;
  actionId: string;
  newValue: ActionKind;
}

export const parseChangeMemberActionKind: NormWireReader<ChangeMemberActionKind> = normWireObject<ChangeMemberActionKind>({ memberId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newValue: normWireRequired(parseActionKind) });
