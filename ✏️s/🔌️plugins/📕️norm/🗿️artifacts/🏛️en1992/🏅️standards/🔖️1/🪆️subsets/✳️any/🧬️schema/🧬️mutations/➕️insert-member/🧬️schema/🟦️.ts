/** ➕️ `insert-member` wire twin: the leaf payload `InsertMember`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type RcMember, parseRcMember } from "../../../📸️snapshot/🟦️.ts";

export interface InsertMember {
  index: number;
  member: RcMember;
}

export const parseInsertMember: NormWireReader<InsertMember> = normWireObject<InsertMember>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), member: normWireRequired(parseRcMember) });
