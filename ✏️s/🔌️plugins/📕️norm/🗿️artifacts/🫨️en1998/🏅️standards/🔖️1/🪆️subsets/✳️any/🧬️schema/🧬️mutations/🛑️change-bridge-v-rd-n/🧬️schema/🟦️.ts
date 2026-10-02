/** 🛑️ `change-bridge-v-rd-n` wire twin: the leaf payload `ChangeBridgeVRdN`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeBridgeVRdN {
  mutation: "changeBridgeVRdN";
  index: number;
  newVRdN: number;
}

export const parseChangeBridgeVRdN: NormWireReader<ChangeBridgeVRdN> = normWireObject<ChangeBridgeVRdN>({ mutation: normWireRequired(normWireLiteral("changeBridgeVRdN")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newVRdN: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
