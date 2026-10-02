/** ⚙️ `change-silo-k` wire twin: the leaf payload `ChangeSiloK`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeSiloK {
  newSiloK: number;
}

export const parseChangeSiloK: NormWireReader<ChangeSiloK> = normWireObject<ChangeSiloK>({ newSiloK: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
