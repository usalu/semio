/** ➕️ `insert-opening` wire twin: the leaf payload `InsertOpening`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { parseWallOpening, type WallOpening } from "../../../📸️snapshot/🟦️.ts";

export interface InsertOpening {
  wallIndex: number;
  index: number;
  opening: WallOpening;
}

export const parseInsertOpening: NormWireReader<InsertOpening> = normWireObject<InsertOpening>({ wallIndex: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), opening: normWireRequired(parseWallOpening) });
