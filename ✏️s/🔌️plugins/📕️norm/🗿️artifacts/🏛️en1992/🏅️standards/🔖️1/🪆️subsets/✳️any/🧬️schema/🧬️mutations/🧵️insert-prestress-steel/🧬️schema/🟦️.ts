/** 🧵 `insert-prestress-steel` wire twin: the leaf payload `InsertPrestressSteel`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type PrestressSteel, parsePrestressSteel } from "../../../📸️snapshot/🟦️.ts";

export interface InsertPrestressSteel {
  index: number;
  prestressSteel: PrestressSteel;
}

export const parseInsertPrestressSteel: NormWireReader<InsertPrestressSteel> = normWireObject<InsertPrestressSteel>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), prestressSteel: normWireRequired(parsePrestressSteel) });
