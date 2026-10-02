/** ➕️ `insert-roofs` wire twin: the leaf payload `InsertRoofs`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { parseRoofArea, type RoofArea } from "../../../📸️snapshot/🟦️.ts";

export interface InsertRoofs {
  index: number;
  item: RoofArea;
}

export const parseInsertRoofs: NormWireReader<InsertRoofs> = normWireObject<InsertRoofs>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), item: normWireRequired(parseRoofArea) });
