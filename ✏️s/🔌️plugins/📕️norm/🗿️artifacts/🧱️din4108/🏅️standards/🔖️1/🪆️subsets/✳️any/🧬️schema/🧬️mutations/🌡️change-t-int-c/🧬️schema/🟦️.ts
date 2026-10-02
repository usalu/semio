/** 🌡️ `change-t-int-c` wire twin: the leaf payload `ChangeTIntC`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeTIntC {
  newTIntC: number;
}

export const parseChangeTIntC: NormWireReader<ChangeTIntC> = normWireObject<ChangeTIntC>({ newTIntC: normWireRequired(normWireRange(normWireNumber, {"exclusiveMinimum":-273.15})) });
