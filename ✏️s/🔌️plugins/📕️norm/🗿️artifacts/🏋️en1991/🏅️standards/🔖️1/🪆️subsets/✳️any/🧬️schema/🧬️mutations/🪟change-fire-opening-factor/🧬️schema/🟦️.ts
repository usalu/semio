/** 🪟 `change-fire-opening-factor` wire twin: the leaf payload `ChangeFireOpeningFactor`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeFireOpeningFactor {
  newFireOpeningFactor: number;
}

export const parseChangeFireOpeningFactor: NormWireReader<ChangeFireOpeningFactor> = normWireObject<ChangeFireOpeningFactor>({ newFireOpeningFactor: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
