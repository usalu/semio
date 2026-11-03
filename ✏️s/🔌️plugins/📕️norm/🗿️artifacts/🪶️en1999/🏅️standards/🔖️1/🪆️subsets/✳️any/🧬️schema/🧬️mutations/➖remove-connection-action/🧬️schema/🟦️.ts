/** 🗂️ `remove-connection-action` wire twin: the leaf payload `RemoveConnectionAction`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveConnectionAction {
  mutation: "removeConnectionAction";
  connectionId: string;
  id: string;
}

export const parseRemoveConnectionAction: NormWireReader<RemoveConnectionAction> = normWireObject<RemoveConnectionAction>({ mutation: normWireRequired(normWireLiteral("removeConnectionAction")), connectionId: normWireRequired(normWireString), id: normWireRequired(normWireString) });
