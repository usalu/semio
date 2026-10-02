/** 🌫️ `change-outdoor-co2` wire twin: the leaf payload `ChangeOutdoorCo2`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeOutdoorCo2 {
  newOutdoorCo2Ppm: number;
}

export const parseChangeOutdoorCo2: NormWireReader<ChangeOutdoorCo2> = normWireObject<ChangeOutdoorCo2>({ newOutdoorCo2Ppm: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
