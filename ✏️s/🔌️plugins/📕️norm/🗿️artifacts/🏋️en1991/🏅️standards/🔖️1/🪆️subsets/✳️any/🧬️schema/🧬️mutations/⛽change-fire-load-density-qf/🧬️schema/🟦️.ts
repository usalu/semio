/** ⛽ `change-fire-load-density-qf` wire twin: the leaf payload `ChangeFireLoadDensityQf`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeFireLoadDensityQf {
  newFireLoadDensityQf: number;
}

export const parseChangeFireLoadDensityQf: NormWireReader<ChangeFireLoadDensityQf> = normWireObject<ChangeFireLoadDensityQf>({ newFireLoadDensityQf: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
