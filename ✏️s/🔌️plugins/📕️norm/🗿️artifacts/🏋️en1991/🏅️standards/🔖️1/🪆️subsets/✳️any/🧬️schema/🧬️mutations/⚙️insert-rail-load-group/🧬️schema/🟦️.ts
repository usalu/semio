/** ⚙ `insert-rail-load-group` wire twin: the leaf payload `InsertRailLoadGroup`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type RailLoadGroup, parseRotatingMachine } from "../../../📸️snapshot/🟦️.ts";

export interface InsertRailLoadGroup {
  index: number;
  item: RailLoadGroup;
}

export const parseInsertRailLoadGroup: NormWireReader<InsertRailLoadGroup> = normWireObject<InsertRailLoadGroup>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), item: normWireRequired(parseRotatingMachine) });
