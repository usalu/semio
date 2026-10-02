/** 🏭️ `change-zone-pollution-class` wire twin: the leaf payload `ChangeZonePollutionClass`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeZonePollutionClass {
  zoneId: string;
  newPollutionClass: string;
}

export const parseChangeZonePollutionClass: NormWireReader<ChangeZonePollutionClass> = normWireObject<ChangeZonePollutionClass>({ zoneId: normWireRequired(normWireString), newPollutionClass: normWireRequired(normWireString) });
