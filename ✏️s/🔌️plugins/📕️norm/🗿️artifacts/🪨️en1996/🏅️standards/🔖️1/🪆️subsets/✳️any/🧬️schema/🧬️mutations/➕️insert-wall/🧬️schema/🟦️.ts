/** ➕️ `insert-wall` wire twin: the leaf payload `InsertWall`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type MasonryWall, parseMasonryWall } from "../../../📸️snapshot/🟦️.ts";

export interface InsertWall {
  index: number;
  wall: MasonryWall;
}

export const parseInsertWall: NormWireReader<InsertWall> = normWireObject<InsertWall>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), wall: normWireRequired(parseMasonryWall) });
