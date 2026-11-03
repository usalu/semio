/** 🌉 `change-bridge-lane` wire twin: the leaf payload `ChangeBridgeLane`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBridgeLane {
  newBridgeLane: number;
}

export const parseChangeBridgeLane: NormWireReader<ChangeBridgeLane> = normWireObject<ChangeBridgeLane>({ newBridgeLane: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":255})) });
