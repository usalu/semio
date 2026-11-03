/** ✏️ `change-bearing-bridge-id` wire twin: the leaf payload `ChangeBearingBridgeId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBearingBridgeId {
  mutation: "changeBearingBridgeId";
  id: string;
  newBridgeId: string;
}

export const parseChangeBearingBridgeId: NormWireReader<ChangeBearingBridgeId> = normWireObject<ChangeBearingBridgeId>({ mutation: normWireRequired(normWireLiteral("changeBearingBridgeId")), id: normWireRequired(normWireString), newBridgeId: normWireRequired(normWireString) });
