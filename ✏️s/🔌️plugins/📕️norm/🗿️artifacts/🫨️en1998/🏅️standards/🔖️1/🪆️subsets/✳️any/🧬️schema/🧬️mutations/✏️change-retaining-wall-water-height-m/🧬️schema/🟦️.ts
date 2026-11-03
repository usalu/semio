/** ✏️ `change-retaining-wall-water-height-m` wire twin: the leaf payload `ChangeRetainingWallWaterHeightM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireNumber, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeRetainingWallWaterHeightM {
  mutation: "changeRetainingWallWaterHeightM";
  index: number;
  newWaterHeightM: number;
}

export const parseChangeRetainingWallWaterHeightM: NormWireReader<ChangeRetainingWallWaterHeightM> = normWireObject<ChangeRetainingWallWaterHeightM>({ mutation: normWireRequired(normWireLiteral("changeRetainingWallWaterHeightM")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newWaterHeightM: normWireRequired(normWireNumber) });
