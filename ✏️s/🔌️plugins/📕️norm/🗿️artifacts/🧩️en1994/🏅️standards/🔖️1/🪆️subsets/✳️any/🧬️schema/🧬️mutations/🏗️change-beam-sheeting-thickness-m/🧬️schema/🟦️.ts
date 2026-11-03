/** 🏗️ `change-beam-sheeting-thickness-m` wire twin: the leaf payload `ChangeBeamSheetingThicknessM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBeamSheetingThicknessM {
  id: string;
  newThicknessM: number;
}

export const parseChangeBeamSheetingThicknessM: NormWireReader<ChangeBeamSheetingThicknessM> = normWireObject<ChangeBeamSheetingThicknessM>({ id: normWireRequired(normWireString), newThicknessM: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
