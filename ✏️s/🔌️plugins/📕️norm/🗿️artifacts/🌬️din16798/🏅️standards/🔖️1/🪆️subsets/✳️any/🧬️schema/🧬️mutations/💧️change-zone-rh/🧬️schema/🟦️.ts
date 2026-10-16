/** 💧️ `change-zone-rh` wire twin: the leaf payload `ChangeZoneRh`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "@semio-tech/norm-js/registry/contract";

export interface ChangeZoneRh {
  zoneId: string;
  newRhPercent: number;
}

export const parseChangeZoneRh: NormWireReader<ChangeZoneRh> = normWireObject<ChangeZoneRh>({ zoneId: normWireRequired(normWireString), newRhPercent: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":100})) });
