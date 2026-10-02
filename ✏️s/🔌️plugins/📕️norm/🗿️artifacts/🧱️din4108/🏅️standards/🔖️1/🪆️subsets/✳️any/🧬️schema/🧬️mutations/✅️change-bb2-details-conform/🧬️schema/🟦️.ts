/** ✅️ `change-bb2-details-conform` wire twin: the leaf payload `ChangeBb2DetailsConform`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireBoolean, normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeBb2DetailsConform {
  newBb2DetailsConform: boolean;
}

export const parseChangeBb2DetailsConform: NormWireReader<ChangeBb2DetailsConform> = normWireObject<ChangeBb2DetailsConform>({ newBb2DetailsConform: normWireRequired(normWireBoolean) });
