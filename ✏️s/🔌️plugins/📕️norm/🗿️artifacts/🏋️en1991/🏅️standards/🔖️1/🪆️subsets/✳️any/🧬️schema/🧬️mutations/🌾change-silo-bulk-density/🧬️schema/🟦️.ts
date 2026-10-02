/** 🌾 `change-silo-bulk-density` wire twin: the leaf payload `ChangeSiloBulkDensity`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeSiloBulkDensity {
  newSiloBulkDensity: number;
}

export const parseChangeSiloBulkDensity: NormWireReader<ChangeSiloBulkDensity> = normWireObject<ChangeSiloBulkDensity>({ newSiloBulkDensity: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
