/** 📍️ `insert-stud` wire twin: the leaf payload `InsertStud`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type Stud, parseStud } from "../../../📸️snapshot/🟦️.ts";

export interface InsertStud {
  index: number;
  item: Stud;
}

export const parseInsertStud: NormWireReader<InsertStud> = normWireObject<InsertStud>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), item: normWireRequired(parseStud) });
