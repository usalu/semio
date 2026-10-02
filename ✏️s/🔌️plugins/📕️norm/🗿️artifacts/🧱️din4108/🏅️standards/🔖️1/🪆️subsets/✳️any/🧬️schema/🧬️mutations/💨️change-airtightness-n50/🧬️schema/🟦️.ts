/** 💨️ `change-airtightness-n50` wire twin: the leaf payload `ChangeAirtightnessN50`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeAirtightnessN50 {
  newAirtightnessN50: number;
}

export const parseChangeAirtightnessN50: NormWireReader<ChangeAirtightnessN50> = normWireObject<ChangeAirtightnessN50>({ newAirtightnessN50: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
