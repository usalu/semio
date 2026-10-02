/** 💨 `change-depth` wire twin: the leaf payload `ChangeDepth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeDepth {
  newDepth: number;
}

export const parseChangeDepth: NormWireReader<ChangeDepth> = normWireObject<ChangeDepth>({ newDepth: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
