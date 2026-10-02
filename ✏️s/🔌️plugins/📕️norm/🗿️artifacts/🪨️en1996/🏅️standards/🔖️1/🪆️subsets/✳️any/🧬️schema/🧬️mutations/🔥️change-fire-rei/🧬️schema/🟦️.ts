/** 🔥️ `change-fire-rei` wire twin: the leaf payload `ChangeFireRei`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeFireRei {
  index: number;
  newFireReiMin: number;
}

export const parseChangeFireRei: NormWireReader<ChangeFireRei> = normWireObject<ChangeFireRei>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newFireReiMin: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
