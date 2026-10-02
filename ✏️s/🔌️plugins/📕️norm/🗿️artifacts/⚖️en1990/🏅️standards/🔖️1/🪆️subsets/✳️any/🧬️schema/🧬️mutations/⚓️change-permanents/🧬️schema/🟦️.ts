/** ⚓️ `change-permanents` wire twin: the leaf payload `ChangePermanents`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireLiteral, normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type En1990PermanentAction, parseEn1990PermanentAction } from "../../../📸️snapshot/🟦️.ts";

export interface ChangePermanents {
  mutation: "changePermanents";
  newPermanents: En1990PermanentAction[];
}

export const parseChangePermanents: NormWireReader<ChangePermanents> = normWireObject<ChangePermanents>({ mutation: normWireRequired(normWireLiteral("changePermanents")), newPermanents: normWireRequired(normWireArray(parseEn1990PermanentAction)) });
