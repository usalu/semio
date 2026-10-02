/** 🌉️ `update-bridge-inputs` wire twin: the leaf payload `UpdateBridgeInputs`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type BridgeFatigue, parseBridgeFatigue } from "../../../📸️snapshot/🟦️.ts";

export interface UpdateBridgeInputs {
  bridgeFatigueItem: BridgeFatigue;
}

export const parseUpdateBridgeInputs: NormWireReader<UpdateBridgeInputs> = normWireObject<UpdateBridgeInputs>({ bridgeFatigueItem: normWireRequired(parseBridgeFatigue) });
