/** 💧 `change-water-head-difference` wire twin: the leaf payload `ChangeWaterHeadDifference`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeWaterHeadDifference {
  mutation: "changeWaterHeadDifference";
  id: string;
  newHeadDifference: number;
}

export const parseChangeWaterHeadDifference: NormWireReader<ChangeWaterHeadDifference> = normWireObject<ChangeWaterHeadDifference>({ mutation: normWireRequired(normWireLiteral("changeWaterHeadDifference")), id: normWireRequired(normWireString), newHeadDifference: normWireRequired(normWireNumber) });
