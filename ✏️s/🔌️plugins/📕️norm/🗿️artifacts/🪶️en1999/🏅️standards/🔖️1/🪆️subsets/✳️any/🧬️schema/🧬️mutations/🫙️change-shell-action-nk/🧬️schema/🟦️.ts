/** 🫙 `change-shell-action-nk` wire twin: the leaf payload `ChangeShellActionNK`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeShellActionNK {
  mutation: "changeShellActionNK";
  shellId: string;
  actionId: string;
  newNK: number;
}

export const parseChangeShellActionNK: NormWireReader<ChangeShellActionNK> = normWireObject<ChangeShellActionNK>({ mutation: normWireRequired(normWireLiteral("changeShellActionNK")), shellId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newNK: normWireRequired(normWireNumber) });
