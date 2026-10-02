/** 🧈 `change-mortar-class` wire twin: the leaf payload `ChangeMortarClass`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type En1996MortarClass, parseEn1996MortarClass } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeMortarClass {
  index: number;
  newMortarClass: En1996MortarClass;
}

export const parseChangeMortarClass: NormWireReader<ChangeMortarClass> = normWireObject<ChangeMortarClass>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newMortarClass: normWireRequired(parseEn1996MortarClass) });
