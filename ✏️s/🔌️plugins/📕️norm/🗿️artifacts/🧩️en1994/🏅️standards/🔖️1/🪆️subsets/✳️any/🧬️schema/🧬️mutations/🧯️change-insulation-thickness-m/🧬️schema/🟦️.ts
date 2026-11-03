/** 🧯️ `change-insulation-thickness-m` wire twin: the leaf payload `ChangeInsulationThicknessM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeInsulationThicknessM {
  newInsulationThicknessM: number;
}

export const parseChangeInsulationThicknessM: NormWireReader<ChangeInsulationThicknessM> = normWireObject<ChangeInsulationThicknessM>({ newInsulationThicknessM: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
