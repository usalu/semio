/** 🌉 `change-bridge-deflection` wire twin: the leaf payload `ChangeBridgeDeflection`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBridgeDeflection {
  mutation: "changeBridgeDeflection";
  id: string;
  newBridgeDeflection: number;
}

export const parseChangeBridgeDeflection: NormWireReader<ChangeBridgeDeflection> = normWireObject<ChangeBridgeDeflection>({ mutation: normWireRequired(normWireLiteral("changeBridgeDeflection")), id: normWireRequired(normWireString), newBridgeDeflection: normWireRequired(normWireNumber) });
