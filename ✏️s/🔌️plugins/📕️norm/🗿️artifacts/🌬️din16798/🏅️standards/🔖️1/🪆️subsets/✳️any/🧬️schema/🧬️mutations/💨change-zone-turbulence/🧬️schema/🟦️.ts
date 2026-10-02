/** 💨 `change-zone-turbulence` wire twin: the leaf payload `ChangeZoneTurbulence`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeZoneTurbulence {
  zoneId: string;
  newTurbulenceIntensityPercent: number;
}

export const parseChangeZoneTurbulence: NormWireReader<ChangeZoneTurbulence> = normWireObject<ChangeZoneTurbulence>({ zoneId: normWireRequired(normWireString), newTurbulenceIntensityPercent: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":100})) });
