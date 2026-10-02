/** 🔎 `change-silo-mu` wire twin: the leaf payload `ChangeSiloMu`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeSiloMu {
  newSiloMu: number;
}

export const parseChangeSiloMu: NormWireReader<ChangeSiloMu> = normWireObject<ChangeSiloMu>({ newSiloMu: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
