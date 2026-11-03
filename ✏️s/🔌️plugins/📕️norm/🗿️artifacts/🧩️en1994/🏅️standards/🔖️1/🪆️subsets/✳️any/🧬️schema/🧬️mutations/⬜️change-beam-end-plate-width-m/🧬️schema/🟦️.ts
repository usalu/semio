/** ⬜️ `change-beam-end-plate-width-m` wire twin: the leaf payload `ChangeBeamEndPlateWidthM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBeamEndPlateWidthM {
  id: string;
  newEndPlateWidthM: number;
}

export const parseChangeBeamEndPlateWidthM: NormWireReader<ChangeBeamEndPlateWidthM> = normWireObject<ChangeBeamEndPlateWidthM>({ id: normWireRequired(normWireString), newEndPlateWidthM: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
