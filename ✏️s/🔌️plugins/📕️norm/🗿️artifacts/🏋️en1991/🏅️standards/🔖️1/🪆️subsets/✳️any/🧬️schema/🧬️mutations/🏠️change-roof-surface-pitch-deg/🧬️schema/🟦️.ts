/** 🏠 `change-roof-surface-pitch-deg` wire twin: the leaf payload `ChangeRoofSurfacePitchDeg`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeRoofSurfacePitchDeg {
  id: string;
  newPitchDeg: number;
}

export const parseChangeRoofSurfacePitchDeg: NormWireReader<ChangeRoofSurfacePitchDeg> = normWireObject<ChangeRoofSurfacePitchDeg>({ id: normWireRequired(normWireString), newPitchDeg: normWireRequired(normWireNumber) });
