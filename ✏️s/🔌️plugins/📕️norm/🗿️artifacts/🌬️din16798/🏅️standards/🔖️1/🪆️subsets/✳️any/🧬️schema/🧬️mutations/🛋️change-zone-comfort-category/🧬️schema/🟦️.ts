/** 🛋️ `change-zone-comfort-category` wire twin: the leaf payload `ChangeZoneComfortCategory`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeZoneComfortCategory {
  zoneId: string;
  newComfortCategory: string;
}

export const parseChangeZoneComfortCategory: NormWireReader<ChangeZoneComfortCategory> = normWireObject<ChangeZoneComfortCategory>({ zoneId: normWireRequired(normWireString), newComfortCategory: normWireRequired(normWireString) });
