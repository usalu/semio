/** 💨️ `change-zone-outdoor-air` wire twin: the leaf payload `ChangeZoneOutdoorAir`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeZoneOutdoorAir {
  zoneId: string;
  newOutdoorAirSuppliedM3H: number;
}

export const parseChangeZoneOutdoorAir: NormWireReader<ChangeZoneOutdoorAir> = normWireObject<ChangeZoneOutdoorAir>({ zoneId: normWireRequired(normWireString), newOutdoorAirSuppliedM3H: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
