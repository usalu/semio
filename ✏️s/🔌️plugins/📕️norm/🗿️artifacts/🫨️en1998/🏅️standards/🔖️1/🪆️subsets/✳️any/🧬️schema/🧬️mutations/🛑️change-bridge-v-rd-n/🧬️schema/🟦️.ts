/** 🛑️ `change-bridge-v-rd-n` wire twin: the leaf payload `ChangeBridgeVRdN`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBridgeVRdN {
  mutation: "changeBridgeVRdN";
  id: string;
  newVRdN: number;
}

export const parseChangeBridgeVRdN: NormWireReader<ChangeBridgeVRdN> = normWireObject<ChangeBridgeVRdN>({ mutation: normWireRequired(normWireLiteral("changeBridgeVRdN")), id: normWireRequired(normWireString), newVRdN: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
