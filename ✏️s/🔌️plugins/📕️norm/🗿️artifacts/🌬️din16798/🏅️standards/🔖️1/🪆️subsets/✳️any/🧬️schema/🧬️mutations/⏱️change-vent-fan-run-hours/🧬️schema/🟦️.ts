/** ⏱️ `change-vent-fan-run-hours` wire twin: the leaf payload `ChangeVentFanRunHours`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `📇️registry/🧬️contract/🏭️generator/🟦️.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "@semio-tech/norm-js/registry/contract";

export interface ChangeVentFanRunHours {
  ventId: string;
  newFanTRunH: number;
}

export const parseChangeVentFanRunHours: NormWireReader<ChangeVentFanRunHours> = normWireObject<ChangeVentFanRunHours>({ ventId: normWireRequired(normWireString), newFanTRunH: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":8760})) });
