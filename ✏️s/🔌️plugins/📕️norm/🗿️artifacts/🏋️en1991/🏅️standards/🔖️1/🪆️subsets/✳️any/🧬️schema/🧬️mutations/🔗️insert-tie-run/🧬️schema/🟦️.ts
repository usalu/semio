/** 🔗 `insert-tie-run` wire twin: the leaf payload `InsertTieRun`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type TieRun, parseTieRun } from "../../../📸️snapshot/🟦️.ts";

export interface InsertTieRun {
  index: number;
  item: TieRun;
}

export const parseInsertTieRun: NormWireReader<InsertTieRun> = normWireObject<InsertTieRun>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), item: normWireRequired(parseTieRun) });
