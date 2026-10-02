/** 🌋️ `change-seismics` wire twin: the leaf payload `ChangeSeismics`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireLiteral, normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type En1990SeismicAction, parseEn1990SeismicAction } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeSeismics {
  mutation: "changeSeismics";
  newSeismics: En1990SeismicAction[];
}

export const parseChangeSeismics: NormWireReader<ChangeSeismics> = normWireObject<ChangeSeismics>({ mutation: normWireRequired(normWireLiteral("changeSeismics")), newSeismics: normWireRequired(normWireArray(parseEn1990SeismicAction)) });
