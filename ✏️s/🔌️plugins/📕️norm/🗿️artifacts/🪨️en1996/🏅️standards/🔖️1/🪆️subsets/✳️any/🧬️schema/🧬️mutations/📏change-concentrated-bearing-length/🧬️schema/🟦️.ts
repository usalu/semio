/** 📏 `change-concentrated-bearing-length` wire twin: the leaf payload `ChangeConcentratedBearingLength`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeConcentratedBearingLength {
  wallIndex: number;
  loadCaseIndex: number;
  index: number;
  newBearingLengthM: number;
}

export const parseChangeConcentratedBearingLength: NormWireReader<ChangeConcentratedBearingLength> = normWireObject<ChangeConcentratedBearingLength>({ wallIndex: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), loadCaseIndex: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newBearingLengthM: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
