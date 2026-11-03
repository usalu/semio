/** 📏 `change-concentrated-bearing-length` wire twin: the leaf payload `ChangeConcentratedBearingLength`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeConcentratedBearingLength {
  wallId: string;
  loadCaseId: string;
  id: string;
  newBearingLengthM: number;
}

export const parseChangeConcentratedBearingLength: NormWireReader<ChangeConcentratedBearingLength> = normWireObject<ChangeConcentratedBearingLength>({ wallId: normWireRequired(normWireString), loadCaseId: normWireRequired(normWireString), id: normWireRequired(normWireString), newBearingLengthM: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":10000})) });
