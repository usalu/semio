/** ➕ `insert-member` wire twin: the leaf payload `InsertMember`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type AluminiumMember, parseAluminiumMember } from "../../../📸️snapshot/🟦️.ts";

export interface InsertMember {
  mutation: "insertMember";
  index: number;
  member: AluminiumMember;
}

export const parseInsertMember: NormWireReader<InsertMember> = normWireObject<InsertMember>({ mutation: normWireRequired(normWireLiteral("insertMember")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), member: normWireRequired(parseAluminiumMember) });
