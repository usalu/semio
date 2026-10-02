/** 🌉 `change-delta-u-wb` wire twin: the leaf payload `ChangeDeltaUWb`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeDeltaUWb {
  mutation: "changeDeltaUWb";
  newDeltaUWbWM2k: number;
}

export const parseChangeDeltaUWb: NormWireReader<ChangeDeltaUWb> = normWireObject<ChangeDeltaUWb>({ mutation: normWireRequired(normWireLiteral("changeDeltaUWb")), newDeltaUWbWM2k: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
