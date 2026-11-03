/** 📐️ `change-concentrated-bearing-area` wire twin: the leaf payload `ChangeConcentratedBearingArea`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeConcentratedBearingArea {
  wallId: string;
  loadCaseId: string;
  id: string;
  newBearingAreaM2: number;
}

export const parseChangeConcentratedBearingArea: NormWireReader<ChangeConcentratedBearingArea> = normWireObject<ChangeConcentratedBearingArea>({ wallId: normWireRequired(normWireString), loadCaseId: normWireRequired(normWireString), id: normWireRequired(normWireString), newBearingAreaM2: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":100000000})) });
