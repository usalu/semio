/** 🌀 `change-assumed-silo-pressure` wire twin: the leaf payload `ChangeAssumedSiloPressure`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeAssumedSiloPressure {
  newAssumedSiloPressure: number;
}

export const parseChangeAssumedSiloPressure: NormWireReader<ChangeAssumedSiloPressure> = normWireObject<ChangeAssumedSiloPressure>({ newAssumedSiloPressure: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
