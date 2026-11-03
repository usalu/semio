/** 🌉 `change-bridge-line-speed` wire twin: the leaf payload `ChangeBridgeLineSpeed`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBridgeLineSpeed {
  mutation: "changeBridgeLineSpeed";
  id: string;
  newLineSpeed: number;
}

export const parseChangeBridgeLineSpeed: NormWireReader<ChangeBridgeLineSpeed> = normWireObject<ChangeBridgeLineSpeed>({ mutation: normWireRequired(normWireLiteral("changeBridgeLineSpeed")), id: normWireRequired(normWireString), newLineSpeed: normWireRequired(normWireNumber) });
