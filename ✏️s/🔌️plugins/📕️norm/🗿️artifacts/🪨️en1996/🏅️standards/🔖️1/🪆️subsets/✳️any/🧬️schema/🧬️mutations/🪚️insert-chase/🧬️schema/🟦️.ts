/** 🪚️ `insert-chase` wire twin: the leaf payload `InsertChase`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type WallChase, parseWallChase } from "../../../📸️snapshot/🟦️.ts";

export interface InsertChase {
  index: number;
  item: WallChase;
}

export const parseInsertChase: NormWireReader<InsertChase> = normWireObject<InsertChase>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), item: normWireRequired(parseWallChase) });
