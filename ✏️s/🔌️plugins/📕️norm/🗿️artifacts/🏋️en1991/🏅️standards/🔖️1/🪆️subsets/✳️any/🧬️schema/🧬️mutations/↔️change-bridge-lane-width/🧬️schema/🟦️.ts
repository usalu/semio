/** ↔️ `change-bridge-lane-width` wire twin: the leaf payload `ChangeBridgeLaneWidth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBridgeLaneWidth {
  newBridgeLaneWidth: number;
}

export const parseChangeBridgeLaneWidth: NormWireReader<ChangeBridgeLaneWidth> = normWireObject<ChangeBridgeLaneWidth>({ newBridgeLaneWidth: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
