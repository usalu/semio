/** 💧️ `change-exposure` wire twin: the leaf payload `ChangeExposure`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type En1996ExposureClass, parseEn1996ExposureClass } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeExposure {
  index: number;
  newExposure: En1996ExposureClass;
}

export const parseChangeExposure: NormWireReader<ChangeExposure> = normWireObject<ChangeExposure>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newExposure: normWireRequired(parseEn1996ExposureClass) });
