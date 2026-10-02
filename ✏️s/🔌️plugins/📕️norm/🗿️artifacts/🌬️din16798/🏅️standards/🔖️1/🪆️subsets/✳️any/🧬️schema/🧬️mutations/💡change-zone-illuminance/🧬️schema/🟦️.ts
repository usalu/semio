/** 💡 `change-zone-illuminance` wire twin: the leaf payload `ChangeZoneIlluminance`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeZoneIlluminance {
  zoneId: string;
  newIlluminanceLx: number;
}

export const parseChangeZoneIlluminance: NormWireReader<ChangeZoneIlluminance> = normWireObject<ChangeZoneIlluminance>({ zoneId: normWireRequired(normWireString), newIlluminanceLx: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
