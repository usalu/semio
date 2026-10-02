/** ♨ `change-assumed-gas-temperature` wire twin: the leaf payload `ChangeAssumedGasTemperature`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeAssumedGasTemperature {
  newAssumedGasTemperature: number;
}

export const parseChangeAssumedGasTemperature: NormWireReader<ChangeAssumedGasTemperature> = normWireObject<ChangeAssumedGasTemperature>({ newAssumedGasTemperature: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
