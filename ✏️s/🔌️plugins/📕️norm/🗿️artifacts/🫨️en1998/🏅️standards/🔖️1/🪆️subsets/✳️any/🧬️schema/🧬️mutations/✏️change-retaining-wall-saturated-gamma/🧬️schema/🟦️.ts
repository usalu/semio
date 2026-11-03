/** ✏️ `change-retaining-wall-saturated-gamma` wire twin: the leaf payload `ChangeRetainingWallSaturatedGamma`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireNumber, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeRetainingWallSaturatedGamma {
  mutation: "changeRetainingWallSaturatedGamma";
  index: number;
  newSaturatedGamma: number;
}

export const parseChangeRetainingWallSaturatedGamma: NormWireReader<ChangeRetainingWallSaturatedGamma> = normWireObject<ChangeRetainingWallSaturatedGamma>({ mutation: normWireRequired(normWireLiteral("changeRetainingWallSaturatedGamma")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newSaturatedGamma: normWireRequired(normWireNumber) });
