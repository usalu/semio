/** 🧯️ `change-insulation-thickness-m` wire twin: the leaf payload `ChangeInsulationThicknessM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeInsulationThicknessM {
  newInsulationThicknessM: number;
}

export const parseChangeInsulationThicknessM: NormWireReader<ChangeInsulationThicknessM> = normWireObject<ChangeInsulationThicknessM>({ newInsulationThicknessM: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
