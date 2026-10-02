/** 🔒️ `change-strict-mode` wire twin: the leaf payload `ChangeStrictMode`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireBoolean, normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeStrictMode {
  newStrictMode: boolean;
}

export const parseChangeStrictMode: NormWireReader<ChangeStrictMode> = normWireObject<ChangeStrictMode>({ newStrictMode: normWireRequired(normWireBoolean) });
