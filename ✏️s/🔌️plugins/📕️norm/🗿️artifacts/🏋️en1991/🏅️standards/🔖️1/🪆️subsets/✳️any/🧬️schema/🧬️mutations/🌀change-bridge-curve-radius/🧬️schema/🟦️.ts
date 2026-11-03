/** 🌀 `change-bridge-curve-radius` wire twin: the leaf payload `ChangeBridgeCurveRadius`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBridgeCurveRadius {
  newBridgeCurveRadius: number;
}

export const parseChangeBridgeCurveRadius: NormWireReader<ChangeBridgeCurveRadius> = normWireObject<ChangeBridgeCurveRadius>({ newBridgeCurveRadius: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
