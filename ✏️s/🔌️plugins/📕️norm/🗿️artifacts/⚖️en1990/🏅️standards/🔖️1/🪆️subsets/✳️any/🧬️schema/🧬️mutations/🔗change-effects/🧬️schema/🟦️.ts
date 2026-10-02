/** 🔗 `change-effects` wire twin: the leaf payload `ChangeEffects`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireLiteral, normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type En1990MemberEffect, parseEn1990MemberEffect } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeEffects {
  mutation: "changeEffects";
  newEffects: En1990MemberEffect[];
}

export const parseChangeEffects: NormWireReader<ChangeEffects> = normWireObject<ChangeEffects>({ mutation: normWireRequired(normWireLiteral("changeEffects")), newEffects: normWireRequired(normWireArray(parseEn1990MemberEffect)) });
