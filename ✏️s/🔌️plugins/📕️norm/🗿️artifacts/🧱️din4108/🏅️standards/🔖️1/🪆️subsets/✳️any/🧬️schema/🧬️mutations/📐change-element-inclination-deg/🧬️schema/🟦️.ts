/** 📐 `change-element-inclination-deg` wire twin: the leaf payload `ChangeElementInclinationDeg`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeElementInclinationDeg {
  elementId: string;
  newInclinationDeg: number;
}

export const parseChangeElementInclinationDeg: NormWireReader<ChangeElementInclinationDeg> = normWireObject<ChangeElementInclinationDeg>({ elementId: normWireRequired(normWireString), newInclinationDeg: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":180})) });
