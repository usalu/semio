/** 👥️ `change-zone-occupants` wire twin: the leaf payload `ChangeZoneOccupants`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeZoneOccupants {
  zoneId: string;
  newOccupants: number;
}

export const parseChangeZoneOccupants: NormWireReader<ChangeZoneOccupants> = normWireObject<ChangeZoneOccupants>({ zoneId: normWireRequired(normWireString), newOccupants: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
