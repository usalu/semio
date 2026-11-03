/** ♨️ `change-assumed-gas-temperature` wire twin: the leaf payload `ChangeAssumedGasTemperature`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAssumedGasTemperature {
  newAssumedGasTemperature: number;
}

export const parseChangeAssumedGasTemperature: NormWireReader<ChangeAssumedGasTemperature> = normWireObject<ChangeAssumedGasTemperature>({ newAssumedGasTemperature: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
