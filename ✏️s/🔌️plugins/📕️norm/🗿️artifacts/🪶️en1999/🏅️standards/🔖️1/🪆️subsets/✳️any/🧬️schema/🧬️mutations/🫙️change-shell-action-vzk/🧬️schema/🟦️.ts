/** 🫙 `change-shell-action-vzk` wire twin: the leaf payload `ChangeShellActionVZK`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeShellActionVZK {
  mutation: "changeShellActionVZK";
  shellId: string;
  actionId: string;
  newVZK: number;
}

export const parseChangeShellActionVZK: NormWireReader<ChangeShellActionVZK> = normWireObject<ChangeShellActionVZK>({ mutation: normWireRequired(normWireLiteral("changeShellActionVZK")), shellId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newVZK: normWireRequired(normWireNumber) });
