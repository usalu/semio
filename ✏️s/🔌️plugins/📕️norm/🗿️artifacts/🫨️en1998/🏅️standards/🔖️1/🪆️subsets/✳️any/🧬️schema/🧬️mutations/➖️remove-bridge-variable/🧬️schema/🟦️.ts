/** ➖️ `remove-bridge-variable` wire twin: the leaf payload `RemoveBridgeVariable`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveBridgeVariable {
  mutation: "removeBridgeVariable";
  bridgeId: string;
  id: string;
}

export const parseRemoveBridgeVariable: NormWireReader<RemoveBridgeVariable> = normWireObject<RemoveBridgeVariable>({ mutation: normWireRequired(normWireLiteral("removeBridgeVariable")), bridgeId: normWireRequired(normWireString), id: normWireRequired(normWireString) });
