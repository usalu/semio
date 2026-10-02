/** 🕳️ `change-vent-duct-leakage` wire twin: the leaf payload `ChangeVentDuctLeakage`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeVentDuctLeakage {
  ventId: string;
  newDuctLeakageM3SM2: number;
}

export const parseChangeVentDuctLeakage: NormWireReader<ChangeVentDuctLeakage> = normWireObject<ChangeVentDuctLeakage>({ ventId: normWireRequired(normWireString), newDuctLeakageM3SM2: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
