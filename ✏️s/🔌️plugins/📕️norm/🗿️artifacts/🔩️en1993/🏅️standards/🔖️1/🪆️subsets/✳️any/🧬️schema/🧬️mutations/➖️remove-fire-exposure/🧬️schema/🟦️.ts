/** ➖️ `remove-fire-exposure` wire twin: the leaf payload `RemoveFireExposure`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface RemoveFireExposure {
  index: number;
}

export const parseRemoveFireExposure: NormWireReader<RemoveFireExposure> = normWireObject<RemoveFireExposure>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
