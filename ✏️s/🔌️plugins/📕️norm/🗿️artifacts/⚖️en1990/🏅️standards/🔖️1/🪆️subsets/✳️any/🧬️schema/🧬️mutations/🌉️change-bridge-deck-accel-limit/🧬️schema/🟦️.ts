/** 🌉 `change-bridge-deck-accel-limit` wire twin: the leaf payload `ChangeBridgeDeckAccelLimit`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBridgeDeckAccelLimit {
  mutation: "changeBridgeDeckAccelLimit";
  id: string;
  newDeckAccelerationLimit: number;
}

export const parseChangeBridgeDeckAccelLimit: NormWireReader<ChangeBridgeDeckAccelLimit> = normWireObject<ChangeBridgeDeckAccelLimit>({ mutation: normWireRequired(normWireLiteral("changeBridgeDeckAccelLimit")), id: normWireRequired(normWireString), newDeckAccelerationLimit: normWireRequired(normWireNumber) });
