/** ➕️ `insert-tower-leg` wire twin: the leaf payload `InsertTowerLeg`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { parseTowerLeg, type TowerLeg } from "../../../📸️snapshot/🟦️.ts";

export interface InsertTowerLeg {
  index: number;
  towerLeg: TowerLeg;
}

export const parseInsertTowerLeg: NormWireReader<InsertTowerLeg> = normWireObject<InsertTowerLeg>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), towerLeg: normWireRequired(parseTowerLeg) });
