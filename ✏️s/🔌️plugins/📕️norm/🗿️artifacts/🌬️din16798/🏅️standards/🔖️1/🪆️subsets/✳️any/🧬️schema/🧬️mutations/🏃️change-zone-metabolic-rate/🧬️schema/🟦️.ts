/** 🏃️ `change-zone-metabolic-rate` wire twin: the leaf payload `ChangeZoneMetabolicRate`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeZoneMetabolicRate {
  zoneId: string;
  newMetabolicRateMet: number;
}

export const parseChangeZoneMetabolicRate: NormWireReader<ChangeZoneMetabolicRate> = normWireObject<ChangeZoneMetabolicRate>({ zoneId: normWireRequired(normWireString), newMetabolicRateMet: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
