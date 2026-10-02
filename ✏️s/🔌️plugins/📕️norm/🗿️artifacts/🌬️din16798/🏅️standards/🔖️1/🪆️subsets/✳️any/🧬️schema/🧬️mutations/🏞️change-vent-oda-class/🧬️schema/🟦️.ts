/** 🏞️ `change-vent-oda-class` wire twin: the leaf payload `ChangeVentOdaClass`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeVentOdaClass {
  ventId: string;
  newOdaClass: string;
}

export const parseChangeVentOdaClass: NormWireReader<ChangeVentOdaClass> = normWireObject<ChangeVentOdaClass>({ ventId: normWireRequired(normWireString), newOdaClass: normWireRequired(normWireString) });
