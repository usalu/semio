/** ✏️ `change-isolation-system-bridge-id` wire twin: the leaf payload `ChangeIsolationSystemBridgeId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeIsolationSystemBridgeId {
  mutation: "changeIsolationSystemBridgeId";
  id: string;
  newBridgeId: string;
}

export const parseChangeIsolationSystemBridgeId: NormWireReader<ChangeIsolationSystemBridgeId> = normWireObject<ChangeIsolationSystemBridgeId>({ mutation: normWireRequired(normWireLiteral("changeIsolationSystemBridgeId")), id: normWireRequired(normWireString), newBridgeId: normWireRequired(normWireString) });
