/** ❄️ `change-cold-formed` wire twin: the leaf payload `ChangeColdFormed`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireLiteral, normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type ColdFormedSheet, parseColdFormedSheet } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeColdFormed {
  mutation: "changeColdFormed";
  coldFormed: ColdFormedSheet[];
}

export const parseChangeColdFormed: NormWireReader<ChangeColdFormed> = normWireObject<ChangeColdFormed>({ mutation: normWireRequired(normWireLiteral("changeColdFormed")), coldFormed: normWireRequired(normWireArray(parseColdFormedSheet)) });
