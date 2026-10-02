/** ⏱ `change-fire-duration` wire twin: the leaf payload `ChangeFireDuration`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeFireDuration {
  newFireDuration: number;
}

export const parseChangeFireDuration: NormWireReader<ChangeFireDuration> = normWireObject<ChangeFireDuration>({ newFireDuration: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
