/** 🧱 `change-unit-group` wire twin: the leaf payload `ChangeUnitGroup`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type En1996UnitGroup, parseEn1996UnitGroup } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeUnitGroup {
  index: number;
  newUnitGroup: En1996UnitGroup;
}

export const parseChangeUnitGroup: NormWireReader<ChangeUnitGroup> = normWireObject<ChangeUnitGroup>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newUnitGroup: normWireRequired(parseEn1996UnitGroup) });
