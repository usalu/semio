/** 🔥️ `change-fire-rating` wire twin: the leaf payload `ChangeFireRating`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type FireRating, parseFireRating } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeFireRating {
  id: string;
  newRating: FireRating;
}

export const parseChangeFireRating: NormWireReader<ChangeFireRating> = normWireObject<ChangeFireRating>({ id: normWireRequired(normWireString), newRating: normWireRequired(parseFireRating) });
