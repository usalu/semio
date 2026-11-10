/** 🟧 `change-zone-internal-gains-w-m2` wire twin: the leaf payload `ChangeZoneInternalGainsWM2`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { type NormWireReader, normWireLiteral, normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString } from "@semio-tech/norm-js/registry/contract";

export interface ChangeZoneInternalGainsWM2 {
  mutation: "changezoneinternalgainswm2";
  zoneId: string;
  newInternalGainsWM2: number;
}

export const parseChangeZoneInternalGainsWM2: NormWireReader<ChangeZoneInternalGainsWM2> = normWireObject<ChangeZoneInternalGainsWM2>({ mutation: normWireRequired(normWireLiteral("changezoneinternalgainswm2")), zoneId: normWireRequired(normWireString), newInternalGainsWM2: normWireRequired(normWireRange(normWireNumber, {"minimum":0.0,"maximum":100.0})) });
