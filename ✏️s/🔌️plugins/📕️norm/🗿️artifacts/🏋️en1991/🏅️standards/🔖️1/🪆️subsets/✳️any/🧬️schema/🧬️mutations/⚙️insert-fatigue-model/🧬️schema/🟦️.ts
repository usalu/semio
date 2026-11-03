/** ⚙ `insert-fatigue-model` wire twin: the leaf payload `InsertFatigueModel`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type FatigueModel, parseFatigueModel } from "../../../📸️snapshot/🟦️.ts";

export interface InsertFatigueModel {
  index: number;
  item: FatigueModel;
}

export const parseInsertFatigueModel: NormWireReader<InsertFatigueModel> = normWireObject<InsertFatigueModel>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), item: normWireRequired(parseFatigueModel) });
