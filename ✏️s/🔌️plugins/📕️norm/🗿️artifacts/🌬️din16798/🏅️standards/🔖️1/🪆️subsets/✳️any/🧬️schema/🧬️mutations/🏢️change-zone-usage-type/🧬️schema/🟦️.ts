/** 🏢️ `change-zone-usage-type` wire twin: the leaf payload `ChangeZoneUsageType`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeZoneUsageType {
  zoneId: string;
  newUsageType: string;
}

export const parseChangeZoneUsageType: NormWireReader<ChangeZoneUsageType> = normWireObject<ChangeZoneUsageType>({ zoneId: normWireRequired(normWireString), newUsageType: normWireRequired(normWireString) });
