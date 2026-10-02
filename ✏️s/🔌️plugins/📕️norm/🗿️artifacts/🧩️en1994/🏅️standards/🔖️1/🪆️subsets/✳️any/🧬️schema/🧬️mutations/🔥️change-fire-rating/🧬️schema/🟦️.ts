/** 🔥️ `change-fire-rating` wire twin: the leaf payload `ChangeFireRating`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeFireRating {
  newFireRating: string;
}

export const parseChangeFireRating: NormWireReader<ChangeFireRating> = normWireObject<ChangeFireRating>({ newFireRating: normWireRequired(normWireString) });
