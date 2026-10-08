/** ➕️ `insert-tower-leg` wire twin: the leaf payload `InsertTowerLeg`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNullable, normWireObject, normWireOptional, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { parseTowerLeg, type TowerLeg } from "../../../📸️snapshot/🟦️.ts";

export interface InsertTowerLeg {
  index?: number | null;
  towerLeg: TowerLeg;
}

export const parseInsertTowerLeg: NormWireReader<InsertTowerLeg> = normWireObject<InsertTowerLeg>({ index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), towerLeg: normWireRequired(parseTowerLeg) });
