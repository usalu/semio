/** 🧱 `change-unit-fb` wire twin: the leaf payload `ChangeUnitFb`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeUnitFb {
  index: number;
  newFBPa: number;
}

export const parseChangeUnitFb: NormWireReader<ChangeUnitFb> = normWireObject<ChangeUnitFb>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newFBPa: normWireRequired(normWireRange(normWireNumber, {"exclusiveMinimum":0})) });
