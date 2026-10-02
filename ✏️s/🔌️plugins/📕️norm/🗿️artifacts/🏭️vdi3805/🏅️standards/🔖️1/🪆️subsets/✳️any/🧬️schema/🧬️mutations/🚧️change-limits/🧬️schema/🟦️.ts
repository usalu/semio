/** 🚧️ `change-limits` wire twin: the leaf payload `ChangeLimits`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { parseSecurityLimits, type SecurityLimits } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeLimits {
  newLimits: SecurityLimits;
}

export const parseChangeLimits: NormWireReader<ChangeLimits> = normWireObject<ChangeLimits>({ newLimits: normWireRequired(parseSecurityLimits) });
