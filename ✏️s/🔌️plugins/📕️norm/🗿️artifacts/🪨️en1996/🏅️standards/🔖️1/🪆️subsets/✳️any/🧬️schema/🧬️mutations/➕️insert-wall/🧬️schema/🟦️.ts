/** ➕️ `insert-wall` wire twin: the leaf payload `InsertWall`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type MasonryWall, parseMasonryWall } from "../../../📸️snapshot/🟦️.ts";

export interface InsertWall {
  index: number;
  wall: MasonryWall;
}

export const parseInsertWall: NormWireReader<InsertWall> = normWireObject<InsertWall>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), wall: normWireRequired(parseMasonryWall) });
