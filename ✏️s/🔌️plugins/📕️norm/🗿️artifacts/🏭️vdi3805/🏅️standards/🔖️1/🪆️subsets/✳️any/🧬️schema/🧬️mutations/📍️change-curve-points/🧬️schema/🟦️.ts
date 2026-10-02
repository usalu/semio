/** 📍️ `change-curve-points` wire twin: the leaf payload `ChangeCurvePoints`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type CurvePoint, parseCurvePoint } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeCurvePoints {
  id: string;
  newPoints: CurvePoint[];
}

export const parseChangeCurvePoints: NormWireReader<ChangeCurvePoints> = normWireObject<ChangeCurvePoints>({ id: normWireRequired(normWireString), newPoints: normWireRequired(normWireArray(parseCurvePoint)) });
