/** ✏️ `change-bridge-variable-qk-n` wire twin: the leaf payload `ChangeBridgeVariableQkN`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBridgeVariableQkN {
  mutation: "changeBridgeVariableQkN";
  bridgeId: string;
  id: string;
  newQkN: number;
}

export const parseChangeBridgeVariableQkN: NormWireReader<ChangeBridgeVariableQkN> = normWireObject<ChangeBridgeVariableQkN>({ mutation: normWireRequired(normWireLiteral("changeBridgeVariableQkN")), bridgeId: normWireRequired(normWireString), id: normWireRequired(normWireString), newQkN: normWireRequired(normWireNumber) });
