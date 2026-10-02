/** 📎 `insert-effect` wire twin: the leaf payload `InsertEffect`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type En1990MemberEffect, parseEn1990MemberEffect } from "../../../📸️snapshot/🟦️.ts";

export interface InsertEffect {
  mutation: "insertEffect";
  index: number;
  item: En1990MemberEffect;
}

export const parseInsertEffect: NormWireReader<InsertEffect> = normWireObject<InsertEffect>({ mutation: normWireRequired(normWireLiteral("insertEffect")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), item: normWireRequired(parseEn1990MemberEffect) });
