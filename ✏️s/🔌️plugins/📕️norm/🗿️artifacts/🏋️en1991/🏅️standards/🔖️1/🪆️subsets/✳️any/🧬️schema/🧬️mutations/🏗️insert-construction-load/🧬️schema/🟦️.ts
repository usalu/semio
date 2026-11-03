/** 🏗 `insert-construction-load` wire twin: the leaf payload `InsertConstructionLoad`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type ConstructionLoad, parseConstructionLoad } from "../../../📸️snapshot/🟦️.ts";

export interface InsertConstructionLoad {
  index: number;
  item: ConstructionLoad;
}

export const parseInsertConstructionLoad: NormWireReader<InsertConstructionLoad> = normWireObject<InsertConstructionLoad>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), item: normWireRequired(parseConstructionLoad) });
