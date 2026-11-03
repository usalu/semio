/** 🌉 `change-bridge-basic-wind-speed` wire twin: the leaf payload `ChangeBridgeBasicWindSpeed`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBridgeBasicWindSpeed {
  mutation: "changeBridgeBasicWindSpeed";
  id: string;
  newBasicWindSpeed: number;
}

export const parseChangeBridgeBasicWindSpeed: NormWireReader<ChangeBridgeBasicWindSpeed> = normWireObject<ChangeBridgeBasicWindSpeed>({ mutation: normWireRequired(normWireLiteral("changeBridgeBasicWindSpeed")), id: normWireRequired(normWireString), newBasicWindSpeed: normWireRequired(normWireNumber) });
