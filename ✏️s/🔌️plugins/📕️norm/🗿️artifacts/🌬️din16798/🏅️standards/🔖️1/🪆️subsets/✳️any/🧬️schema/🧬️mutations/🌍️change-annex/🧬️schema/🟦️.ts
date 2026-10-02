/** 🌍️ `change-annex` wire twin: the leaf payload `ChangeAnnex`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type Din16798AnnexChoice, parseDin16798AnnexChoice } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeAnnex {
  newAnnex: Din16798AnnexChoice;
}

export const parseChangeAnnex: NormWireReader<ChangeAnnex> = normWireObject<ChangeAnnex>({ newAnnex: normWireRequired(parseDin16798AnnexChoice) });
