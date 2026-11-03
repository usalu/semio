/** 🧭️ `change-building-plan-regular` wire twin: the leaf payload `ChangeBuildingPlanRegular`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireBoolean, normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBuildingPlanRegular {
  mutation: "changeBuildingPlanRegular";
  id: string;
  newPlanRegular: boolean;
}

export const parseChangeBuildingPlanRegular: NormWireReader<ChangeBuildingPlanRegular> = normWireObject<ChangeBuildingPlanRegular>({ mutation: normWireRequired(normWireLiteral("changeBuildingPlanRegular")), id: normWireRequired(normWireString), newPlanRegular: normWireRequired(normWireBoolean) });
