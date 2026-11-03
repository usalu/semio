/** ⬛️ `change-beam-steel-area-m2` wire twin: the leaf payload `ChangeBeamSteelAreaM2`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBeamSteelAreaM2 {
  id: string;
  newAM2: number;
}

export const parseChangeBeamSteelAreaM2: NormWireReader<ChangeBeamSteelAreaM2> = normWireObject<ChangeBeamSteelAreaM2>({ id: normWireRequired(normWireString), newAM2: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
