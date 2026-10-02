/** 🗼️ `update-tower-inputs` wire twin: the leaf payload `UpdateTowerInputs`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { parseTowerLeg, type TowerLeg } from "../../../📸️snapshot/🟦️.ts";

export interface UpdateTowerInputs {
  towerLeg: TowerLeg;
}

export const parseUpdateTowerInputs: NormWireReader<UpdateTowerInputs> = normWireObject<UpdateTowerInputs>({ towerLeg: normWireRequired(parseTowerLeg) });
