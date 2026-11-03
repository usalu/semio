/** ✏️ `change-retaining-wall-soil-gamma` wire twin: the leaf payload `ChangeRetainingWallSoilGamma`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeRetainingWallSoilGamma {
  mutation: "changeRetainingWallSoilGamma";
  id: string;
  newSoilGamma: number;
}

export const parseChangeRetainingWallSoilGamma: NormWireReader<ChangeRetainingWallSoilGamma> = normWireObject<ChangeRetainingWallSoilGamma>({ mutation: normWireRequired(normWireLiteral("changeRetainingWallSoilGamma")), id: normWireRequired(normWireString), newSoilGamma: normWireRequired(normWireNumber) });
