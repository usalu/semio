/** ➕️ `insert-roof` wire twin: the leaf payload `InsertRoof`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type RoofArea, parseRoofArea } from "../../../📸️snapshot/🟦️.ts";

export interface InsertRoof {
  index: number;
  item: RoofArea;
}

export const parseInsertRoof: NormWireReader<InsertRoof> = normWireObject<InsertRoof>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), item: normWireRequired(parseRoofArea) });
