/** 🧱 `change-fire-thermal-inertia` wire twin: the leaf payload `ChangeFireThermalInertia`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeFireThermalInertia {
  newFireThermalInertia: number;
}

export const parseChangeFireThermalInertia: NormWireReader<ChangeFireThermalInertia> = normWireObject<ChangeFireThermalInertia>({ newFireThermalInertia: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
