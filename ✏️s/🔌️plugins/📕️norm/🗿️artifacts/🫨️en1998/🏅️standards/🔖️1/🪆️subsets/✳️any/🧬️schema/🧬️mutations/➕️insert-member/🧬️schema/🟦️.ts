/** ➕️ `insert-member` wire twin: the leaf payload `InsertMember`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireNullable, normWireObject, normWireOptional, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type En1998Member, parseEn1998Member } from "../../../📸️snapshot/🟦️.ts";

export interface InsertMember {
  mutation: "insertMember";
  buildingId: string;
  index?: number | null;
  member: En1998Member;
}

export const parseInsertMember: NormWireReader<InsertMember> = normWireObject<InsertMember>({ mutation: normWireRequired(normWireLiteral("insertMember")), buildingId: normWireRequired(normWireString), index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), member: normWireRequired(parseEn1998Member) });
