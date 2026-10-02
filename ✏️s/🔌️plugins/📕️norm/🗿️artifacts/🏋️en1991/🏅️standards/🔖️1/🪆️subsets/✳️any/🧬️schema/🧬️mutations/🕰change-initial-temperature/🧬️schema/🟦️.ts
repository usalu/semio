/** 🕰 `change-initial-temperature` wire twin: the leaf payload `ChangeInitialTemperature`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeInitialTemperature {
  newT0: number;
}

export const parseChangeInitialTemperature: NormWireReader<ChangeInitialTemperature> = normWireObject<ChangeInitialTemperature>({ newT0: normWireRequired(normWireNumber) });
