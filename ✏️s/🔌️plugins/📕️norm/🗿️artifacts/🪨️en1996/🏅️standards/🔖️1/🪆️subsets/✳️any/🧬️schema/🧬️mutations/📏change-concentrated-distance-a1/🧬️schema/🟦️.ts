/** 📏 `change-concentrated-distance-a1` wire twin: the leaf payload `ChangeConcentratedDistanceA1`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeConcentratedDistanceA1 {
  wallId: string;
  loadCaseId: string;
  id: string;
  newDistanceA1M: number;
}

export const parseChangeConcentratedDistanceA1: NormWireReader<ChangeConcentratedDistanceA1> = normWireObject<ChangeConcentratedDistanceA1>({ wallId: normWireRequired(normWireString), loadCaseId: normWireRequired(normWireString), id: normWireRequired(normWireString), newDistanceA1M: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":10000})) });
