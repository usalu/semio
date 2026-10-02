/** ➕️ `insert-beam` wire twin: the leaf payload `InsertBeam`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type CompositeBeam, parseCompositeBeam } from "../../../📸️snapshot/🟦️.ts";

export interface InsertBeam {
  index: number;
  beam: CompositeBeam;
}

export const parseInsertBeam: NormWireReader<InsertBeam> = normWireObject<InsertBeam>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), beam: normWireRequired(parseCompositeBeam) });
