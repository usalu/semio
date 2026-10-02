/** 📏 `change-linear-temperature-gradient` wire twin: the leaf payload `ChangeLinearTemperatureGradient`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeLinearTemperatureGradient {
  newDeltaTM: number;
}

export const parseChangeLinearTemperatureGradient: NormWireReader<ChangeLinearTemperatureGradient> = normWireObject<ChangeLinearTemperatureGradient>({ newDeltaTM: normWireRequired(normWireNumber) });
