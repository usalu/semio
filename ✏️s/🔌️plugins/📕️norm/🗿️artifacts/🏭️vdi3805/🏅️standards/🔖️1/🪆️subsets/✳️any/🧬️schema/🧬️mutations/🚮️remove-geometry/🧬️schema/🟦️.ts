/** 🚮️ `remove-geometry` wire twin: the leaf payload `RemoveGeometry`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface RemoveGeometry {
  id: string;
}

export const parseRemoveGeometry: NormWireReader<RemoveGeometry> = normWireObject<RemoveGeometry>({ id: normWireRequired(normWireString) });
