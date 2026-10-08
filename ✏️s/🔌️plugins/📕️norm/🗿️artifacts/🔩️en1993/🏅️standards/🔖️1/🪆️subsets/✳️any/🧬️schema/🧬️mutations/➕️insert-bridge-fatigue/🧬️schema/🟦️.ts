/** ➕️ `insert-bridge-fatigue` wire twin: the leaf payload `InsertBridgeFatigue`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNullable, normWireObject, normWireOptional, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type BridgeFatigue, parseBridgeFatigue } from "../../../📸️snapshot/🟦️.ts";

export interface InsertBridgeFatigue {
  index?: number | null;
  bridgeFatigueItem: BridgeFatigue;
}

export const parseInsertBridgeFatigue: NormWireReader<InsertBridgeFatigue> = normWireObject<InsertBridgeFatigue>({ index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), bridgeFatigueItem: normWireRequired(parseBridgeFatigue) });
