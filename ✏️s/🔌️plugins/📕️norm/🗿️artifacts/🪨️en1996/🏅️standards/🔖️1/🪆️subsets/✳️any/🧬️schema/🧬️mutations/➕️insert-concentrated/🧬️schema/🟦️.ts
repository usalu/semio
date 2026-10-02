/** ➕️ `insert-concentrated` wire twin: the leaf payload `InsertConcentrated`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type ConcentratedLoad, parseConcentratedLoad } from "../../../📸️snapshot/🟦️.ts";

export interface InsertConcentrated {
  wallIndex: number;
  loadCaseIndex: number;
  index: number;
  load: ConcentratedLoad;
}

export const parseInsertConcentrated: NormWireReader<InsertConcentrated> = normWireObject<InsertConcentrated>({ wallIndex: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), loadCaseIndex: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), load: normWireRequired(parseConcentratedLoad) });
