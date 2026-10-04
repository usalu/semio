/** 🏙️ `change-storey-count` wire twin: the leaf payload `ChangeStoreyCount`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeStoreyCount {
  newStoreyCount: number;
}

export const parseChangeStoreyCount: NormWireReader<ChangeStoreyCount> = normWireObject<ChangeStoreyCount>({ newStoreyCount: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
