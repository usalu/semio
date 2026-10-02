/** ☀️ `change-zone-window-g-value` wire twin: the leaf payload `ChangeZoneWindowGValue`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeZoneWindowGValue {
  zoneId: string;
  windowId: string;
  newGValue: number;
}

export const parseChangeZoneWindowGValue: NormWireReader<ChangeZoneWindowGValue> = normWireObject<ChangeZoneWindowGValue>({ zoneId: normWireRequired(normWireString), windowId: normWireRequired(normWireString), newGValue: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":1})) });
