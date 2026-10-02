/** 📉️ `remove-curve` wire twin: the leaf payload `RemoveCurve`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface RemoveCurve {
  id: string;
}

export const parseRemoveCurve: NormWireReader<RemoveCurve> = normWireObject<RemoveCurve>({ id: normWireRequired(normWireString) });
