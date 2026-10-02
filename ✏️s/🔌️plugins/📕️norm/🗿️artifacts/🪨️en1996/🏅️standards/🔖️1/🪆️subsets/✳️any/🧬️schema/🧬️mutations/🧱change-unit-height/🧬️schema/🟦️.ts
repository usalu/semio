/** 🧱 `change-unit-height` wire twin: the leaf payload `ChangeUnitHeight`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeUnitHeight {
  index: number;
  newUnitHeightM: number;
}

export const parseChangeUnitHeight: NormWireReader<ChangeUnitHeight> = normWireObject<ChangeUnitHeight>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newUnitHeightM: normWireRequired(normWireRange(normWireNumber, {"exclusiveMinimum":0})) });
