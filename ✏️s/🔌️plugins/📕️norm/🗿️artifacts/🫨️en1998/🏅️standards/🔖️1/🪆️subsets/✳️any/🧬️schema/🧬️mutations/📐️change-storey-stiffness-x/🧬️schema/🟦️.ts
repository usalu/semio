/** 📐️ `change-storey-stiffness-x` wire twin: the leaf payload `ChangeStoreyStiffnessX`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeStoreyStiffnessX {
  mutation: "changeStoreyStiffnessX";
  buildingIndex: number;
  storeyIndex: number;
  newStiffnessX: number;
}

export const parseChangeStoreyStiffnessX: NormWireReader<ChangeStoreyStiffnessX> = normWireObject<ChangeStoreyStiffnessX>({ mutation: normWireRequired(normWireLiteral("changeStoreyStiffnessX")), buildingIndex: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), storeyIndex: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newStiffnessX: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
