/** 🌉️ `change-bridge-second-lane-traffic` wire twin: the leaf payload `ChangeBridgeSecondLaneTraffic`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBridgeSecondLaneTraffic {
  id: string;
  newSecondLaneTraffic: number;
}

export const parseChangeBridgeSecondLaneTraffic: NormWireReader<ChangeBridgeSecondLaneTraffic> = normWireObject<ChangeBridgeSecondLaneTraffic>({ id: normWireRequired(normWireString), newSecondLaneTraffic: normWireRequired(normWireNumber) });
