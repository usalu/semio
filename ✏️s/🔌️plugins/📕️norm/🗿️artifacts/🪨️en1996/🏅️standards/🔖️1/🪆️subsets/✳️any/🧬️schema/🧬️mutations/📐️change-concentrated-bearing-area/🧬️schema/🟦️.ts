/** 📐️ `change-concentrated-bearing-area` wire twin: the leaf payload `ChangeConcentratedBearingArea`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeConcentratedBearingArea {
  wallIndex: number;
  loadCaseIndex: number;
  index: number;
  newBearingAreaM2: number;
}

export const parseChangeConcentratedBearingArea: NormWireReader<ChangeConcentratedBearingArea> = normWireObject<ChangeConcentratedBearingArea>({ wallIndex: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), loadCaseIndex: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newBearingAreaM2: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
