/** 🛤 `insert-rail-dynamic` wire twin: the leaf payload `InsertRailDynamic`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type RailDynamic, parseRailDynamic } from "../../../📸️snapshot/🟦️.ts";

export interface InsertRailDynamic {
  index: number;
  item: RailDynamic;
}

export const parseInsertRailDynamic: NormWireReader<InsertRailDynamic> = normWireObject<InsertRailDynamic>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), item: normWireRequired(parseRailDynamic) });
