/** 🏗️ `insert-construction-stage` wire twin: the leaf payload `InsertConstructionStage`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type ConstructionStage, parseConstructionStage } from "../../../📸️snapshot/🟦️.ts";

export interface InsertConstructionStage {
  index: number;
  item: ConstructionStage;
}

export const parseInsertConstructionStage: NormWireReader<InsertConstructionStage> = normWireObject<InsertConstructionStage>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), item: normWireRequired(parseConstructionStage) });
