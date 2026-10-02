/** ➕ `insert-permanent` wire twin: the leaf payload `InsertPermanent`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type En1990PermanentAction, parseEn1990PermanentAction } from "../../../📸️snapshot/🟦️.ts";

export interface InsertPermanent {
  mutation: "insertPermanent";
  index: number;
  item: En1990PermanentAction;
}

export const parseInsertPermanent: NormWireReader<InsertPermanent> = normWireObject<InsertPermanent>({ mutation: normWireRequired(normWireLiteral("insertPermanent")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), item: normWireRequired(parseEn1990PermanentAction) });
