/** ✏️ `change-pier-base-bridge-id` wire twin: the leaf payload `ChangePierBaseBridgeId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePierBaseBridgeId {
  mutation: "changePierBaseBridgeId";
  id: string;
  newBridgeId: string;
}

export const parseChangePierBaseBridgeId: NormWireReader<ChangePierBaseBridgeId> = normWireObject<ChangePierBaseBridgeId>({ mutation: normWireRequired(normWireLiteral("changePierBaseBridgeId")), id: normWireRequired(normWireString), newBridgeId: normWireRequired(normWireString) });
