/** 🚫️ `remove-zone-window` wire twin: the leaf payload `RemoveZoneWindow`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface RemoveZoneWindow {
  zoneId: string;
  index: number;
}

export const parseRemoveZoneWindow: NormWireReader<RemoveZoneWindow> = normWireObject<RemoveZoneWindow>({ zoneId: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
