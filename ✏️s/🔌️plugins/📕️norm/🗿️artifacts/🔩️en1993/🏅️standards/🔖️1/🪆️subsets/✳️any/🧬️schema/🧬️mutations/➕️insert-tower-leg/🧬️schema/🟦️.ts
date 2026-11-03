/** ➕️ `insert-tower-leg` wire twin: the leaf payload `InsertTowerLeg`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNullable, normWireObject, normWireOptional, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type TowerLeg, parseTowerLeg } from "../../../📸️snapshot/🟦️.ts";

export interface InsertTowerLeg {
  index?: number | null;
  towerLeg: TowerLeg;
}

export const parseInsertTowerLeg: NormWireReader<InsertTowerLeg> = normWireObject<InsertTowerLeg>({ index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), towerLeg: normWireRequired(parseTowerLeg) });
