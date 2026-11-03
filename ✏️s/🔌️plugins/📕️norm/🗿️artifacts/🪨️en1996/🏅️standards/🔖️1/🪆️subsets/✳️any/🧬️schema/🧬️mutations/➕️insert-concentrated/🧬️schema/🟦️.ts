/** ➕️ `insert-concentrated` wire twin: the leaf payload `InsertConcentrated`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type ConcentratedLoad, parseConcentratedLoad } from "../../../📸️snapshot/🟦️.ts";

export interface InsertConcentrated {
  wallId: string;
  loadCaseId: string;
  index: number;
  load: ConcentratedLoad;
}

export const parseInsertConcentrated: NormWireReader<InsertConcentrated> = normWireObject<InsertConcentrated>({ wallId: normWireRequired(normWireString), loadCaseId: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), load: normWireRequired(parseConcentratedLoad) });
