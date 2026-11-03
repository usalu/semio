/** 🔩 `insert-reinforcement-grade` wire twin: the leaf payload `InsertReinforcementGrade`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type ReinforcementGrade, parseReinforcementGrade } from "../../../📸️snapshot/🟦️.ts";

export interface InsertReinforcementGrade {
  index: number;
  reinforcementGrade: ReinforcementGrade;
}

export const parseInsertReinforcementGrade: NormWireReader<InsertReinforcementGrade> = normWireObject<InsertReinforcementGrade>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), reinforcementGrade: normWireRequired(parseReinforcementGrade) });
