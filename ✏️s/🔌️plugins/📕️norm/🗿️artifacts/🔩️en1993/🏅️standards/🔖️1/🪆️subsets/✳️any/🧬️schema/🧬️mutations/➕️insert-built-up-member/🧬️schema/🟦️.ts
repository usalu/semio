/** ➕️ `insert-built-up-member` wire twin: the leaf payload `InsertBuiltUpMember`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNullable, normWireObject, normWireOptional, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type BuiltUpMember, parseBuiltUpMember } from "../../../📸️snapshot/🟦️.ts";

export interface InsertBuiltUpMember {
  index?: number | null;
  builtUpMember: BuiltUpMember;
}

export const parseInsertBuiltUpMember: NormWireReader<InsertBuiltUpMember> = normWireObject<InsertBuiltUpMember>({ index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), builtUpMember: normWireRequired(parseBuiltUpMember) });
