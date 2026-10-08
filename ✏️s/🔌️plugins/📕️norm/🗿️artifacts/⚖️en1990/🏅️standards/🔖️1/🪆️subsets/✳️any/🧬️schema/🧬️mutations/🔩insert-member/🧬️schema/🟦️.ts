/** 🔩 `insert-member` wire twin: the leaf payload `InsertMember`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireNullable, normWireObject, normWireOptional, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type En1990Member, parseEn1990Member } from "../../../📸️snapshot/🟦️.ts";

export interface InsertMember {
  mutation: "insertMember";
  index?: number | null;
  item: En1990Member;
}

export const parseInsertMember: NormWireReader<InsertMember> = normWireObject<InsertMember>({ mutation: normWireRequired(normWireLiteral("insertMember")), index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), item: normWireRequired(parseEn1990Member) });
