/** 🌀 `change-cellar-ventilation` wire twin: the leaf payload `ChangeCellarVentilation`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeCellarVentilation {
  newCellarVentilationM3H: number;
}

export const parseChangeCellarVentilation: NormWireReader<ChangeCellarVentilation> = normWireObject<ChangeCellarVentilation>({ newCellarVentilationM3H: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
