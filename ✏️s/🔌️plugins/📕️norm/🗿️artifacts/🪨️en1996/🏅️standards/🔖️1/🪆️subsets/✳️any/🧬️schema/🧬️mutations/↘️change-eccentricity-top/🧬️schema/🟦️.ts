/** ↘️ `change-eccentricity-top` wire twin: the leaf payload `ChangeEccentricityTop`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeEccentricityTop {
  index: number;
  newEccentricityTopM: number;
}

export const parseChangeEccentricityTop: NormWireReader<ChangeEccentricityTop> = normWireObject<ChangeEccentricityTop>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newEccentricityTopM: normWireRequired(normWireNumber) });
