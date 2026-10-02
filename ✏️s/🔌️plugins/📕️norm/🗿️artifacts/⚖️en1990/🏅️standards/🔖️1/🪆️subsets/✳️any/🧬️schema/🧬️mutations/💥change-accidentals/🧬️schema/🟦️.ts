/** 💥 `change-accidentals` wire twin: the leaf payload `ChangeAccidentals`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireLiteral, normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type En1990AccidentalAction, parseEn1990AccidentalAction } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeAccidentals {
  mutation: "changeAccidentals";
  newAccidentals: En1990AccidentalAction[];
}

export const parseChangeAccidentals: NormWireReader<ChangeAccidentals> = normWireObject<ChangeAccidentals>({ mutation: normWireRequired(normWireLiteral("changeAccidentals")), newAccidentals: normWireRequired(normWireArray(parseEn1990AccidentalAction)) });
