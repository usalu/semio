/** 🏔️ `change-north-german-lowland-snow` wire twin: the leaf payload `ChangeNorthGermanLowlandSnow`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireBoolean, normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeNorthGermanLowlandSnow {
  newNorthGermanLowlandSnow: boolean;
}

export const parseChangeNorthGermanLowlandSnow: NormWireReader<ChangeNorthGermanLowlandSnow> = normWireObject<ChangeNorthGermanLowlandSnow>({ newNorthGermanLowlandSnow: normWireRequired(normWireBoolean) });
