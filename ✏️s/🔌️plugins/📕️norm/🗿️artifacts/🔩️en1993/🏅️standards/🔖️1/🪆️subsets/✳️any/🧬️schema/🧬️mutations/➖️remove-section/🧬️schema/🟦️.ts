/** ➖️ `remove-section` wire twin: the leaf payload `RemoveSection`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface RemoveSection {
  index: number;
}

export const parseRemoveSection: NormWireReader<RemoveSection> = normWireObject<RemoveSection>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
