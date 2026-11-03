/** 🗂️ `remove-connection` wire twin: the leaf payload `RemoveConnection`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveConnection {
  mutation: "removeConnection";
  id: string;
}

export const parseRemoveConnection: NormWireReader<RemoveConnection> = normWireObject<RemoveConnection>({ mutation: normWireRequired(normWireLiteral("removeConnection")), id: normWireRequired(normWireString) });
