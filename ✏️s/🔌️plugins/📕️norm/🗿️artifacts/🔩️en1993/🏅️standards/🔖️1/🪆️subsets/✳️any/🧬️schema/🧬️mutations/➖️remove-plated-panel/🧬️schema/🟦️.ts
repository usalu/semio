/** ➖️ `remove-plated-panel` wire twin: the leaf payload `RemovePlatedPanel`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface RemovePlatedPanel {
  index: number;
}

export const parseRemovePlatedPanel: NormWireReader<RemovePlatedPanel> = normWireObject<RemovePlatedPanel>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
