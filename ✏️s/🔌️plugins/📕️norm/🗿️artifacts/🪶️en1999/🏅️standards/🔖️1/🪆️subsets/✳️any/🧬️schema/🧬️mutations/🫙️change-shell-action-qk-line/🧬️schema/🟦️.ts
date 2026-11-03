/** 🫙 `change-shell-action-qk-line` wire twin: the leaf payload `ChangeShellActionQKLine`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeShellActionQKLine {
  mutation: "changeShellActionQKLine";
  shellId: string;
  actionId: string;
  newQKLine: number;
}

export const parseChangeShellActionQKLine: NormWireReader<ChangeShellActionQKLine> = normWireObject<ChangeShellActionQKLine>({ mutation: normWireRequired(normWireLiteral("changeShellActionQKLine")), shellId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newQKLine: normWireRequired(normWireNumber) });
