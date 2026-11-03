/** 🫙 `change-shell-action-gk-line` wire twin: the leaf payload `ChangeShellActionGKLine`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeShellActionGKLine {
  mutation: "changeShellActionGKLine";
  shellId: string;
  actionId: string;
  newGKLine: number;
}

export const parseChangeShellActionGKLine: NormWireReader<ChangeShellActionGKLine> = normWireObject<ChangeShellActionGKLine>({ mutation: normWireRequired(normWireLiteral("changeShellActionGKLine")), shellId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newGKLine: normWireRequired(normWireNumber) });
