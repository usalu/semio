/** ♻️ `change-vent-heat-recovery` wire twin: the leaf payload `ChangeVentHeatRecovery`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeVentHeatRecovery {
  ventId: string;
  newHeatRecoveryEta: number;
}

export const parseChangeVentHeatRecovery: NormWireReader<ChangeVentHeatRecovery> = normWireObject<ChangeVentHeatRecovery>({ ventId: normWireRequired(normWireString), newHeatRecoveryEta: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":1})) });
