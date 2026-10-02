/** 📐️ `change-net-floor-area-m2` wire twin: the leaf payload `ChangeNetFloorAreaM2`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeNetFloorAreaM2 {
  mutation: "changeNetFloorAreaM2";
  newNetFloorAreaM2: number;
}

export const parseChangeNetFloorAreaM2: NormWireReader<ChangeNetFloorAreaM2> = normWireObject<ChangeNetFloorAreaM2>({ mutation: normWireRequired(normWireLiteral("changeNetFloorAreaM2")), newNetFloorAreaM2: normWireRequired(normWireRange(normWireNumber, {"exclusiveMinimum":0})) });
