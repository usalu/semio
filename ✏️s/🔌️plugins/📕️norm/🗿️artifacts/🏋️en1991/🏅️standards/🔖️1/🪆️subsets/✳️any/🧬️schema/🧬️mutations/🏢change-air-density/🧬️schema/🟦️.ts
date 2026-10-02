/** 🏢 `change-air-density` wire twin: the leaf payload `ChangeAirDensity`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeAirDensity {
  newAirDensity: number;
}

export const parseChangeAirDensity: NormWireReader<ChangeAirDensity> = normWireObject<ChangeAirDensity>({ newAirDensity: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
