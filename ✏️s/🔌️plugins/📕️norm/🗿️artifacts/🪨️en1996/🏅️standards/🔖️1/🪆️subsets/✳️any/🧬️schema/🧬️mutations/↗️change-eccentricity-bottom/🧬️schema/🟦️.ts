/** ↗️ `change-eccentricity-bottom` wire twin: the leaf payload `ChangeEccentricityBottom`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeEccentricityBottom {
  index: number;
  newEccentricityBottomM: number;
}

export const parseChangeEccentricityBottom: NormWireReader<ChangeEccentricityBottom> = normWireObject<ChangeEccentricityBottom>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newEccentricityBottomM: normWireRequired(normWireNumber) });
