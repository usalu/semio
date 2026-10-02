/** 🌍️ `change-annex` wire twin: the leaf payload `ChangeAnnex`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type AnnexChoice, parseAnnexChoice } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeAnnex {
  mutation: "changeAnnex";
  newAnnex: AnnexChoice;
}

export const parseChangeAnnex: NormWireReader<ChangeAnnex> = normWireObject<ChangeAnnex>({ mutation: normWireRequired(normWireLiteral("changeAnnex")), newAnnex: normWireRequired(parseAnnexChoice) });
