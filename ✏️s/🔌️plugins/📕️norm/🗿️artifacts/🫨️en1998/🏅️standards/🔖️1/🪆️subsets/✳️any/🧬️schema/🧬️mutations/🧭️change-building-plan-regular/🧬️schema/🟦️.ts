/** 🧭️ `change-building-plan-regular` wire twin: the leaf payload `ChangeBuildingPlanRegular`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireBoolean, normWireInteger, normWireLiteral, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeBuildingPlanRegular {
  mutation: "changeBuildingPlanRegular";
  buildingIndex: number;
  newPlanRegular: boolean;
}

export const parseChangeBuildingPlanRegular: NormWireReader<ChangeBuildingPlanRegular> = normWireObject<ChangeBuildingPlanRegular>({ mutation: normWireRequired(normWireLiteral("changeBuildingPlanRegular")), buildingIndex: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newPlanRegular: normWireRequired(normWireBoolean) });
