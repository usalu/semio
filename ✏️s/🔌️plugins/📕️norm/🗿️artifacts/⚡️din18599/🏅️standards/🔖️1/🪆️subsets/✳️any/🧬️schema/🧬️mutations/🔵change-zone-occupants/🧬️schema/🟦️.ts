/** 🔵 `change-zone-occupants` wire twin: the leaf payload `ChangeZoneOccupants`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { type NormWireReader, normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, normWireString } from "@semio-tech/norm-js/registry/contract";

export interface ChangeZoneOccupants {
  mutation: "changezoneoccupants";
  zoneId: string;
  newOccupants: number;
}

export const parseChangeZoneOccupants: NormWireReader<ChangeZoneOccupants> = normWireObject<ChangeZoneOccupants>({ mutation: normWireRequired(normWireLiteral("changezoneoccupants")), zoneId: normWireRequired(normWireString), newOccupants: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":10000})) });
