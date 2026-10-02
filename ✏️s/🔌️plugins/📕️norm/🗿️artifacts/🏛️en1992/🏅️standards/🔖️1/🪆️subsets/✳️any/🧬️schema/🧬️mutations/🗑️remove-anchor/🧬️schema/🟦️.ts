/** 🗑️ `remove-anchor` wire twin: the leaf payload `RemoveAnchor`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface RemoveAnchor {
  anchorId: string;
}

export const parseRemoveAnchor: NormWireReader<RemoveAnchor> = normWireObject<RemoveAnchor>({ anchorId: normWireRequired(normWireString) });
