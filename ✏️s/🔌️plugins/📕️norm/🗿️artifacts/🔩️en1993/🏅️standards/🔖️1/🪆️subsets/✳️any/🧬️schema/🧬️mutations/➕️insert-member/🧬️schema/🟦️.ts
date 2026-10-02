/** ➕️ `insert-member` wire twin: the leaf payload `InsertMember`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { parseSteelMember, type SteelMember } from "../../../📸️snapshot/🟦️.ts";

export interface InsertMember {
  index: number;
  member: SteelMember;
}

export const parseInsertMember: NormWireReader<InsertMember> = normWireObject<InsertMember>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), member: normWireRequired(parseSteelMember) });
