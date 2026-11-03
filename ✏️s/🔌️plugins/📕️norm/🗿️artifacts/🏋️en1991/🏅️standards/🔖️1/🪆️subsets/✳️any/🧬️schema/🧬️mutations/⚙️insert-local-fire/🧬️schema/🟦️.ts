/** ⚙ `insert-local-fire` wire twin: the leaf payload `InsertLocalFire`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type LocalFire, parseLocalFire } from "../../../📸️snapshot/🟦️.ts";

export interface InsertLocalFire {
  index: number;
  item: LocalFire;
}

export const parseInsertLocalFire: NormWireReader<InsertLocalFire> = normWireObject<InsertLocalFire>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), item: normWireRequired(parseLocalFire) });
