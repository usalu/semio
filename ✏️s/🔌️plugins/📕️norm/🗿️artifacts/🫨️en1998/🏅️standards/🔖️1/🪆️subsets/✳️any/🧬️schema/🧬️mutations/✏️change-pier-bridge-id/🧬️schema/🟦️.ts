/** ✏️ `change-pier-bridge-id` wire twin: the leaf payload `ChangePierBridgeId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePierBridgeId {
  mutation: "changePierBridgeId";
  id: string;
  newBridgeId: string;
}

export const parseChangePierBridgeId: NormWireReader<ChangePierBridgeId> = normWireObject<ChangePierBridgeId>({ mutation: normWireRequired(normWireLiteral("changePierBridgeId")), id: normWireRequired(normWireString), newBridgeId: normWireRequired(normWireString) });
