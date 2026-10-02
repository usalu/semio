/** 🏠 `change-width` wire twin: the leaf payload `ChangeWidth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeWidth {
  newWidth: number;
}

export const parseChangeWidth: NormWireReader<ChangeWidth> = normWireObject<ChangeWidth>({ newWidth: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
