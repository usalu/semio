/** ⚙️ `change-vent-system-type` wire twin: the leaf payload `ChangeVentSystemType`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeVentSystemType {
  ventId: string;
  newSystemType: string;
}

export const parseChangeVentSystemType: NormWireReader<ChangeVentSystemType> = normWireObject<ChangeVentSystemType>({ ventId: normWireRequired(normWireString), newSystemType: normWireRequired(normWireString) });
