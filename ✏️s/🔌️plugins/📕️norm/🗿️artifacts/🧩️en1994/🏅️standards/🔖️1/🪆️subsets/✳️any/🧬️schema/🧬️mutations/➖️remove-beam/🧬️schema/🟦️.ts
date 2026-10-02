/** ➖️ `remove-beam` wire twin: the leaf payload `RemoveBeam`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface RemoveBeam {
  index: number;
}

export const parseRemoveBeam: NormWireReader<RemoveBeam> = normWireObject<RemoveBeam>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
