/** 🌉 `change-bridge-end-rotation` wire twin: the leaf payload `ChangeBridgeEndRotation`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBridgeEndRotation {
  mutation: "changeBridgeEndRotation";
  id: string;
  newEndRotation: number;
}

export const parseChangeBridgeEndRotation: NormWireReader<ChangeBridgeEndRotation> = normWireObject<ChangeBridgeEndRotation>({ mutation: normWireRequired(normWireLiteral("changeBridgeEndRotation")), id: normWireRequired(normWireString), newEndRotation: normWireRequired(normWireNumber) });
