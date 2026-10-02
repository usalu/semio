/** 🧱 `change-unit-length` wire twin: the leaf payload `ChangeUnitLength`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeUnitLength {
  index: number;
  newUnitLengthM: number;
}

export const parseChangeUnitLength: NormWireReader<ChangeUnitLength> = normWireObject<ChangeUnitLength>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newUnitLengthM: normWireRequired(normWireRange(normWireNumber, {"exclusiveMinimum":0})) });
