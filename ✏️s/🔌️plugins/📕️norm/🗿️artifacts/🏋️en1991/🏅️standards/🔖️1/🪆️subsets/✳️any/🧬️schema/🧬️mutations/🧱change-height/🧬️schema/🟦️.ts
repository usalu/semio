/** 🧱 `change-height` wire twin: the leaf payload `ChangeHeight`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeHeight {
  newHeight: number;
}

export const parseChangeHeight: NormWireReader<ChangeHeight> = normWireObject<ChangeHeight>({ newHeight: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
