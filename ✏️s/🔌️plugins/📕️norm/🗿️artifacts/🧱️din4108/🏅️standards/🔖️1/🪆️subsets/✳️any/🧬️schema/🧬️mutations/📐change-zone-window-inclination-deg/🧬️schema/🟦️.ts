/** 📐 `change-zone-window-inclination-deg` wire twin: the leaf payload `ChangeZoneWindowInclinationDeg`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeZoneWindowInclinationDeg {
  zoneId: string;
  windowId: string;
  newInclinationDeg: number;
}

export const parseChangeZoneWindowInclinationDeg: NormWireReader<ChangeZoneWindowInclinationDeg> = normWireObject<ChangeZoneWindowInclinationDeg>({ zoneId: normWireRequired(normWireString), windowId: normWireRequired(normWireString), newInclinationDeg: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":180})) });
