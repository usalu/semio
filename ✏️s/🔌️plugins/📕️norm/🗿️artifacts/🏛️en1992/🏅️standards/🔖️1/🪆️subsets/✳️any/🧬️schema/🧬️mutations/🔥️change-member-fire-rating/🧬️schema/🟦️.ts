/** 🔥️ `change-member-fire-rating` wire twin: the leaf payload `ChangeMemberFireRating`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type FireRating, parseFireRating } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeMemberFireRating {
  memberId: string;
  newRating: FireRating;
}

export const parseChangeMemberFireRating: NormWireReader<ChangeMemberFireRating> = normWireObject<ChangeMemberFireRating>({ memberId: normWireRequired(normWireString), newRating: normWireRequired(parseFireRating) });
