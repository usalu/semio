/** 🏠 `insert-roof-surface` wire twin: the leaf payload `InsertRoofSurface`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type RoofSurface, parseRoofSurface } from "../../../📸️snapshot/🟦️.ts";

export interface InsertRoofSurface {
  index: number;
  item: RoofSurface;
}

export const parseInsertRoofSurface: NormWireReader<InsertRoofSurface> = normWireObject<InsertRoofSurface>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), item: normWireRequired(parseRoofSurface) });
