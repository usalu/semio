/** ➕️ `insert-member` wire twin: the leaf payload `InsertMember`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNullable, normWireObject, normWireOptional, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type SteelMember, parseSteelMember } from "../../../📸️snapshot/🟦️.ts";

export interface InsertMember {
  index?: number | null;
  member: SteelMember;
}

export const parseInsertMember: NormWireReader<InsertMember> = normWireObject<InsertMember>({ index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), member: normWireRequired(parseSteelMember) });
