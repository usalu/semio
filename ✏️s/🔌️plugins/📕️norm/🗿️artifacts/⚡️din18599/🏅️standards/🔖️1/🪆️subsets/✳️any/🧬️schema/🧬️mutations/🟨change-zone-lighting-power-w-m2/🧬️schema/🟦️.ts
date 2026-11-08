/** 🟨 `change-zone-lighting-power-w-m2` wire twin: the leaf payload `ChangeZoneLightingPowerWM2`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { type NormWireReader, normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString } from "@semio-tech/norm-js/registry/contract";

export interface ChangeZoneLightingPowerWM2 {
  mutation: "changezonelightingpowerwm2";
  zoneId: string;
  newLightingPowerWM2: number;
}

export const parseChangeZoneLightingPowerWM2: NormWireReader<ChangeZoneLightingPowerWM2> = normWireObject<ChangeZoneLightingPowerWM2>({ mutation: normWireRequired(normWireLiteral("changezonelightingpowerwm2")), zoneId: normWireRequired(normWireString), newLightingPowerWM2: normWireRequired(normWireNumber) });
