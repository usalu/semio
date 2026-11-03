/** 🏞️ `change-soil-height` wire twin: the leaf payload `ChangeSoilHeight`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSoilHeight {
  id: string;
  newSoilHeightM: number;
}

export const parseChangeSoilHeight: NormWireReader<ChangeSoilHeight> = normWireObject<ChangeSoilHeight>({ id: normWireRequired(normWireString), newSoilHeightM: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":10000})) });
