/** 🧱 `change-unit-material` wire twin: the leaf payload `ChangeUnitMaterial`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type En1996UnitMaterial, parseEn1996UnitMaterial } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeUnitMaterial {
  index: number;
  newUnitMaterial: En1996UnitMaterial;
}

export const parseChangeUnitMaterial: NormWireReader<ChangeUnitMaterial> = normWireObject<ChangeUnitMaterial>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newUnitMaterial: normWireRequired(parseEn1996UnitMaterial) });
