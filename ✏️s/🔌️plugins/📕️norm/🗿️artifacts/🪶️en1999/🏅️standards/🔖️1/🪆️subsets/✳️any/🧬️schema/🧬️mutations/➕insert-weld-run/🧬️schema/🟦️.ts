/** 🗂️ `insert-weld-run` wire twin: the leaf payload `InsertWeldRun`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type WeldRun, parseWeldRun } from "../../../📸️snapshot/🟦️.ts";

export interface InsertWeldRun {
  mutation: "insertWeldRun";
  index: number;
  weldRun: WeldRun;
}

export const parseInsertWeldRun: NormWireReader<InsertWeldRun> = normWireObject<InsertWeldRun>({ mutation: normWireRequired(normWireLiteral("insertWeldRun")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), weldRun: normWireRequired(parseWeldRun) });
