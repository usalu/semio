/** 🗑️ `remove-vent-system` wire twin: the leaf payload `RemoveVentSystem`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface RemoveVentSystem {
  ventId: string;
}

export const parseRemoveVentSystem: NormWireReader<RemoveVentSystem> = normWireObject<RemoveVentSystem>({ ventId: normWireRequired(normWireString) });
