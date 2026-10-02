/** ❄️ `change-en-sk` wire twin: the leaf payload `ChangeEnSk`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeEnSk {
  newEnSk: number;
}

export const parseChangeEnSk: NormWireReader<ChangeEnSk> = normWireObject<ChangeEnSk>({ newEnSk: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
