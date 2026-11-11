/** 🟧 `change-zone-internal-gains-w-m2` wire twin: the leaf payload `ChangeZoneInternalGainsWM2`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `📇️registry/🧬️contract/🏭️generator/🟦️.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "@semio-tech/norm-js/registry/contract";

export interface ChangeZoneInternalGainsWM2 {
  mutation: "changezoneinternalgainswm2";
  zoneId: string;
  newInternalGainsWM2: number;
}

export const parseChangeZoneInternalGainsWM2: NormWireReader<ChangeZoneInternalGainsWM2> = normWireObject<ChangeZoneInternalGainsWM2>({ mutation: normWireRequired(normWireLiteral("changezoneinternalgainswm2")), zoneId: normWireRequired(normWireString), newInternalGainsWM2: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":100})) });
