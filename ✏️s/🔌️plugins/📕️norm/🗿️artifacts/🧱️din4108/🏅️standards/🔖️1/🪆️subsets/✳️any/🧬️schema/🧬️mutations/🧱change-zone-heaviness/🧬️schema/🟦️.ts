/** 🧱 `change-zone-heaviness` wire twin: the leaf payload `ChangeZoneHeaviness`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeZoneHeaviness {
  zoneId: string;
  newHeaviness: string;
}

export const parseChangeZoneHeaviness: NormWireReader<ChangeZoneHeaviness> = normWireObject<ChangeZoneHeaviness>({ zoneId: normWireRequired(normWireString), newHeaviness: normWireRequired(normWireString) });
