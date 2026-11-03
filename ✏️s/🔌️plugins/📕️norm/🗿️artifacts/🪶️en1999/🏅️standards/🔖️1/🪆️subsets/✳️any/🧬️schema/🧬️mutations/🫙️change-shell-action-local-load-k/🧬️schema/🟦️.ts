/** 🫙 `change-shell-action-local-load-k` wire twin: the leaf payload `ChangeShellActionLocalLoadK`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeShellActionLocalLoadK {
  mutation: "changeShellActionLocalLoadK";
  shellId: string;
  actionId: string;
  newLocalLoadK: number;
}

export const parseChangeShellActionLocalLoadK: NormWireReader<ChangeShellActionLocalLoadK> = normWireObject<ChangeShellActionLocalLoadK>({ mutation: normWireRequired(normWireLiteral("changeShellActionLocalLoadK")), shellId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newLocalLoadK: normWireRequired(normWireNumber) });
