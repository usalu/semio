/** 🧱️ `change-beam-steel-web-thickness-m` wire twin: the leaf payload `ChangeBeamSteelWebThicknessM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBeamSteelWebThicknessM {
  id: string;
  newTwM: number;
}

export const parseChangeBeamSteelWebThicknessM: NormWireReader<ChangeBeamSteelWebThicknessM> = normWireObject<ChangeBeamSteelWebThicknessM>({ id: normWireRequired(normWireString), newTwM: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
