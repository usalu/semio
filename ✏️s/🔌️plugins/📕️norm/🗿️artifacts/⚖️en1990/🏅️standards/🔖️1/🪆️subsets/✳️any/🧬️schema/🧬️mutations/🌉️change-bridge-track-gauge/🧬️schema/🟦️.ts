/** 🌉 `change-bridge-track-gauge` wire twin: the leaf payload `ChangeBridgeTrackGauge`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBridgeTrackGauge {
  mutation: "changeBridgeTrackGauge";
  id: string;
  newTrackGauge: number;
}

export const parseChangeBridgeTrackGauge: NormWireReader<ChangeBridgeTrackGauge> = normWireObject<ChangeBridgeTrackGauge>({ mutation: normWireRequired(normWireLiteral("changeBridgeTrackGauge")), id: normWireRequired(normWireString), newTrackGauge: normWireRequired(normWireNumber) });
