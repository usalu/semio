/** 🧱 `change-unit-width` wire twin: the leaf payload `ChangeUnitWidth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeUnitWidth {
  index: number;
  newUnitWidthM: number;
}

export const parseChangeUnitWidth: NormWireReader<ChangeUnitWidth> = normWireObject<ChangeUnitWidth>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newUnitWidthM: normWireRequired(normWireRange(normWireNumber, {"exclusiveMinimum":0})) });
