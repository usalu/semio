/** 📏️ `change-delta-c-dev` wire twin: the leaf payload `ChangeDeltaCDev`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeDeltaCDev {
  newDeltaCDev: number;
}

export const parseChangeDeltaCDev: NormWireReader<ChangeDeltaCDev> = normWireObject<ChangeDeltaCDev>({ newDeltaCDev: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
