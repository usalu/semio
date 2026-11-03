/** ✏️ `change-building-storey-stiffness-basis` wire twin: the leaf payload `ChangeBuildingStoreyStiffnessBasis`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBuildingStoreyStiffnessBasis {
  mutation: "changeBuildingStoreyStiffnessBasis";
  id: string;
  newStoreyStiffnessBasis: "simplified" | "declared";
}

export const parseChangeBuildingStoreyStiffnessBasis: NormWireReader<ChangeBuildingStoreyStiffnessBasis> = normWireObject<ChangeBuildingStoreyStiffnessBasis>({ mutation: normWireRequired(normWireLiteral("changeBuildingStoreyStiffnessBasis")), id: normWireRequired(normWireString), newStoreyStiffnessBasis: normWireRequired(normWireLiteral("simplified", "declared")) });
