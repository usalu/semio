/** 🔩️ `change-connection-steel-plate` wire twin: the leaf payload `ChangeConnectionSteelPlate`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireBoolean, normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeConnectionSteelPlate {
  connectionId: string;
  newValue: boolean;
}

export const parseChangeConnectionSteelPlate: NormWireReader<ChangeConnectionSteelPlate> = normWireObject<ChangeConnectionSteelPlate>({ connectionId: normWireRequired(normWireString), newValue: normWireRequired(normWireBoolean) });
