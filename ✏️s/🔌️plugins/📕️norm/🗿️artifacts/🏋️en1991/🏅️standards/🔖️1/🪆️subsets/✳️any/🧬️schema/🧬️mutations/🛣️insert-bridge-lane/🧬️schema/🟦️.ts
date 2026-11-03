/** 🛣️ `insert-bridge-lane` wire twin: the leaf payload `InsertBridgeLane`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type BridgeLane, parseBridgeLane } from "../../../📸️snapshot/🟦️.ts";

export interface InsertBridgeLane {
  index: number;
  item: BridgeLane;
}

export const parseInsertBridgeLane: NormWireReader<InsertBridgeLane> = normWireObject<InsertBridgeLane>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), item: normWireRequired(parseBridgeLane) });
