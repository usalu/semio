/** ➕️ `insert-cold-formed-member` wire twin: the leaf payload `InsertColdFormedMember`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNullable, normWireObject, normWireOptional, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type ColdFormedMember, parseColdFormedMember } from "../../../📸️snapshot/🟦️.ts";

export interface InsertColdFormedMember {
  index?: number | null;
  coldFormedMember: ColdFormedMember;
}

export const parseInsertColdFormedMember: NormWireReader<InsertColdFormedMember> = normWireObject<InsertColdFormedMember>({ index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), coldFormedMember: normWireRequired(parseColdFormedMember) });
