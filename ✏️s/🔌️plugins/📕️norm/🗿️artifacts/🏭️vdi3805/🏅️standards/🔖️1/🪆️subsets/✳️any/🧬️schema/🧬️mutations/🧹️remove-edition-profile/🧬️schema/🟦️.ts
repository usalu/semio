/** 🧹️ `remove-edition-profile` wire twin: the leaf payload `RemoveEditionProfile`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface RemoveEditionProfile {
  sheet: string;
}

export const parseRemoveEditionProfile: NormWireReader<RemoveEditionProfile> = normWireObject<RemoveEditionProfile>({ sheet: normWireRequired(normWireString) });
