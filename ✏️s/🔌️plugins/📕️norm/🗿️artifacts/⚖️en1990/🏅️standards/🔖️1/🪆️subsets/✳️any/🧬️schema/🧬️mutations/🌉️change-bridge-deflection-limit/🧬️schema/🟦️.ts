/** 🌉 `change-bridge-deflection-limit` wire twin: the leaf payload `ChangeBridgeDeflectionLimit`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBridgeDeflectionLimit {
  mutation: "changeBridgeDeflectionLimit";
  id: string;
  newBridgeDeflectionLimit: number;
}

export const parseChangeBridgeDeflectionLimit: NormWireReader<ChangeBridgeDeflectionLimit> = normWireObject<ChangeBridgeDeflectionLimit>({ mutation: normWireRequired(normWireLiteral("changeBridgeDeflectionLimit")), id: normWireRequired(normWireString), newBridgeDeflectionLimit: normWireRequired(normWireNumber) });
