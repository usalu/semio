/** 🏋️ `change-concentrated-force` wire twin: the leaf payload `ChangeConcentratedForce`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeConcentratedForce {
  wallIndex: number;
  loadCaseIndex: number;
  index: number;
  newForceN: number;
}

export const parseChangeConcentratedForce: NormWireReader<ChangeConcentratedForce> = normWireObject<ChangeConcentratedForce>({ wallIndex: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), loadCaseIndex: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newForceN: normWireRequired(normWireNumber) });
