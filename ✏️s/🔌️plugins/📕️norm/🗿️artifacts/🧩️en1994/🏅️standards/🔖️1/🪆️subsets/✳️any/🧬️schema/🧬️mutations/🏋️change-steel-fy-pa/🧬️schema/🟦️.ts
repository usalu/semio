/** 🏋️ `change-steel-fy-pa` wire twin: the leaf payload `ChangeSteelFYPa`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeSteelFYPa {
  newSteelFYPa: number;
}

export const parseChangeSteelFYPa: NormWireReader<ChangeSteelFYPa> = normWireObject<ChangeSteelFYPa>({ newSteelFYPa: normWireRequired(normWireRange(normWireNumber, {"exclusiveMinimum":0})) });
