/** 🧱 `change-density` wire twin: the leaf payload `ChangeDensity`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeDensity {
  index: number;
  newDensityKgM3: number;
}

export const parseChangeDensity: NormWireReader<ChangeDensity> = normWireObject<ChangeDensity>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newDensityKgM3: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
