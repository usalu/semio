/** 🧈 `change-mortar-type` wire twin: the leaf payload `ChangeMortarType`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type En1996MortarType, parseEn1996MortarType } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeMortarType {
  index: number;
  newMortarType: En1996MortarType;
}

export const parseChangeMortarType: NormWireReader<ChangeMortarType> = normWireObject<ChangeMortarType>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newMortarType: normWireRequired(parseEn1996MortarType) });
