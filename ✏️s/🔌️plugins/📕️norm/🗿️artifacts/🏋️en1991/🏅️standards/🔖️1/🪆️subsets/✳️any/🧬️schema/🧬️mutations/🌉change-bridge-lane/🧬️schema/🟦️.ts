/** 🌉 `change-bridge-lane` wire twin: the leaf payload `ChangeBridgeLane`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeBridgeLane {
  newBridgeLane: number;
}

export const parseChangeBridgeLane: NormWireReader<ChangeBridgeLane> = normWireObject<ChangeBridgeLane>({ newBridgeLane: normWireRequired(normWireRange(normWireInteger, {"minimum":1})) });
