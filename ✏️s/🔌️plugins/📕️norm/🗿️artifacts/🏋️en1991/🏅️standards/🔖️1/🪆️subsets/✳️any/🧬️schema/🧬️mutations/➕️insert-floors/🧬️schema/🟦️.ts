/** ➕️ `insert-floors` wire twin: the leaf payload `InsertFloors`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type FloorArea, parseFloorArea } from "../../../📸️snapshot/🟦️.ts";

export interface InsertFloors {
  index: number;
  item: FloorArea;
}

export const parseInsertFloors: NormWireReader<InsertFloors> = normWireObject<InsertFloors>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), item: normWireRequired(parseFloorArea) });
