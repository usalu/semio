/** ➕ `insert-footing` wire twin: the leaf payload `InsertFooting`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { parseSpreadFoundation, type SpreadFoundation } from "../../../📸️snapshot/🟦️.ts";

export interface InsertFooting {
  mutation: "insertFooting";
  index: number;
  footing: SpreadFoundation;
}

export const parseInsertFooting: NormWireReader<InsertFooting> = normWireObject<InsertFooting>({ mutation: normWireRequired(normWireLiteral("insertFooting")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), footing: normWireRequired(parseSpreadFoundation) });
