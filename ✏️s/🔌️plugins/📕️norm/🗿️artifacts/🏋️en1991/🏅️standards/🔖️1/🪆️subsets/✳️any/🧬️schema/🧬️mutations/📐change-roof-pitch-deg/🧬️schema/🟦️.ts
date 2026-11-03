/** 📐 `change-roof-pitch-deg` wire twin: the leaf payload `ChangeRoofPitchDeg`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeRoofPitchDeg {
  id: string;
  newPitchDeg: number;
}

export const parseChangeRoofPitchDeg: NormWireReader<ChangeRoofPitchDeg> = normWireObject<ChangeRoofPitchDeg>({ id: normWireRequired(normWireString), newPitchDeg: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":90})) });
