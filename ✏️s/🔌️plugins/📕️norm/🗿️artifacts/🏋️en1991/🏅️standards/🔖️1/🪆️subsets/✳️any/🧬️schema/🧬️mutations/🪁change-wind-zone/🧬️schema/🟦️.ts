/** 🪁 `change-wind-zone` wire twin: the leaf payload `ChangeWindZone`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeWindZone {
  newWindZone: number;
}

export const parseChangeWindZone: NormWireReader<ChangeWindZone> = normWireObject<ChangeWindZone>({ newWindZone: normWireRequired(normWireRange(normWireInteger, {"minimum":1,"maximum":4})) });
