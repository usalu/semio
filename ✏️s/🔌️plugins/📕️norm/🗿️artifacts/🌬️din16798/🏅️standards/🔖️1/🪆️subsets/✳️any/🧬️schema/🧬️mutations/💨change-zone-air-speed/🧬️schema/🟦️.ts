/** 💨 `change-zone-air-speed` wire twin: the leaf payload `ChangeZoneAirSpeed`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeZoneAirSpeed {
  zoneId: string;
  newAirSpeedMS: number;
}

export const parseChangeZoneAirSpeed: NormWireReader<ChangeZoneAirSpeed> = normWireObject<ChangeZoneAirSpeed>({ zoneId: normWireRequired(normWireString), newAirSpeedMS: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
