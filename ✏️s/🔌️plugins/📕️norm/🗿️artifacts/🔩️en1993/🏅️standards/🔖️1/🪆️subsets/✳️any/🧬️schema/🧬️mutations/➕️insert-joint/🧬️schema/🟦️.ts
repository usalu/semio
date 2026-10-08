/** ➕️ `insert-joint` wire twin: the leaf payload `InsertJoint`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNullable, normWireObject, normWireOptional, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { parseSteelJoint, type SteelJoint } from "../../../📸️snapshot/🟦️.ts";

export interface InsertJoint {
  index?: number | null;
  joint: SteelJoint;
}

export const parseInsertJoint: NormWireReader<InsertJoint> = normWireObject<InsertJoint>({ index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), joint: normWireRequired(parseSteelJoint) });
