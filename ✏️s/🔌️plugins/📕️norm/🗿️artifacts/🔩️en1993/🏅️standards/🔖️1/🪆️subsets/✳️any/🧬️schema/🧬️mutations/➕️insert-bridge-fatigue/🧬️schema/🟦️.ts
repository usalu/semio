/** ➕️ `insert-bridge-fatigue` wire twin: the leaf payload `InsertBridgeFatigue`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNullable, normWireObject, normWireOptional, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type BridgeFatigue, parseBridgeFatigue } from "../../../📸️snapshot/🟦️.ts";

export interface InsertBridgeFatigue {
  index?: number | null;
  bridgeFatigueItem: BridgeFatigue;
}

export const parseInsertBridgeFatigue: NormWireReader<InsertBridgeFatigue> = normWireObject<InsertBridgeFatigue>({ index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), bridgeFatigueItem: normWireRequired(parseBridgeFatigue) });
