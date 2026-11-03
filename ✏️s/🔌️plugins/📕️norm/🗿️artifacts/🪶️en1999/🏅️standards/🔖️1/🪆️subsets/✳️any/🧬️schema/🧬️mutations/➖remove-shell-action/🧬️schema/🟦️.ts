/** 🗂️ `remove-shell-action` wire twin: the leaf payload `RemoveShellAction`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveShellAction {
  mutation: "removeShellAction";
  shellId: string;
  id: string;
}

export const parseRemoveShellAction: NormWireReader<RemoveShellAction> = normWireObject<RemoveShellAction>({ mutation: normWireRequired(normWireLiteral("removeShellAction")), shellId: normWireRequired(normWireString), id: normWireRequired(normWireString) });
