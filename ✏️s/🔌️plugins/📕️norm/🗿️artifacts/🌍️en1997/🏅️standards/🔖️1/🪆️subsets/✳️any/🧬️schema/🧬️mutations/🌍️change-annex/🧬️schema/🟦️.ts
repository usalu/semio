/** 🌍️ `change-annex` wire twin: the leaf payload `ChangeAnnex`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type En1997AnnexChoice, parseEn1997AnnexChoice } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeAnnex {
  mutation: "changeAnnex";
  newAnnex: En1997AnnexChoice;
}

export const parseChangeAnnex: NormWireReader<ChangeAnnex> = normWireObject<ChangeAnnex>({ mutation: normWireRequired(normWireLiteral("changeAnnex")), newAnnex: normWireRequired(parseEn1997AnnexChoice) });
