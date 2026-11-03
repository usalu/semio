/** 🔥️ `change-member-fire-rating` wire twin: the leaf payload `ChangeMemberFireRating`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type FireRating, parseFireRating } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeMemberFireRating {
  memberId: string;
  newRating: FireRating;
}

export const parseChangeMemberFireRating: NormWireReader<ChangeMemberFireRating> = normWireObject<ChangeMemberFireRating>({ memberId: normWireRequired(normWireString), newRating: normWireRequired(parseFireRating) });
