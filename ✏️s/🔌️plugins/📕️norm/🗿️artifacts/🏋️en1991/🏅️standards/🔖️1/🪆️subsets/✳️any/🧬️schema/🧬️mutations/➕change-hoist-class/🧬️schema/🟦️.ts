/** ➕ `change-hoist-class` wire twin: the leaf payload `ChangeHoistClass`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeHoistClass {
  newHoistClass: string;
}

export const parseChangeHoistClass: NormWireReader<ChangeHoistClass> = normWireObject<ChangeHoistClass>({ newHoistClass: normWireRequired(normWireString) });
