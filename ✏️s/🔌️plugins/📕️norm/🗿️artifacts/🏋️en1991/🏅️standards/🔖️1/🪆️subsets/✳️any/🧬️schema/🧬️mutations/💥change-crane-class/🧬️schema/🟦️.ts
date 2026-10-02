/** 💥 `change-crane-class` wire twin: the leaf payload `ChangeCraneClass`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeCraneClass {
  newCraneClass: string;
}

export const parseChangeCraneClass: NormWireReader<ChangeCraneClass> = normWireObject<ChangeCraneClass>({ newCraneClass: normWireRequired(normWireString) });
