/** 🔖️ `change-edition-profile` wire twin: the leaf payload `ChangeEditionProfile`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { parseVdi3805EditionProfileChoice, type Vdi3805EditionProfileChoice } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeEditionProfile {
  sheet: string;
  newChoice: Vdi3805EditionProfileChoice;
}

export const parseChangeEditionProfile: NormWireReader<ChangeEditionProfile> = normWireObject<ChangeEditionProfile>({ sheet: normWireRequired(normWireString), newChoice: normWireRequired(parseVdi3805EditionProfileChoice) });
