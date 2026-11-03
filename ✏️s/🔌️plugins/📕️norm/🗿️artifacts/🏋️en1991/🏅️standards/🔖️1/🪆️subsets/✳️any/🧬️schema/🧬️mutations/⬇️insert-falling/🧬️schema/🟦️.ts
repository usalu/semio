/** ⬇ `insert-falling` wire twin: the leaf payload `InsertFalling`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type FallingLoad, parseFallingLoad } from "../../../📸️snapshot/🟦️.ts";

export interface InsertFalling {
  index: number;
  item: FallingLoad;
}

export const parseInsertFalling: NormWireReader<InsertFalling> = normWireObject<InsertFalling>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), item: normWireRequired(parseFallingLoad) });
