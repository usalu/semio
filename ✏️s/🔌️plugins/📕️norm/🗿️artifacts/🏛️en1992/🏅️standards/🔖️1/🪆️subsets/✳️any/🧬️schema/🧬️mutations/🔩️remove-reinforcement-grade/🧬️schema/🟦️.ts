/** 🔩 `remove-reinforcement-grade` wire twin: the leaf payload `RemoveReinforcementGrade`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveReinforcementGrade {
  index: number;
}

export const parseRemoveReinforcementGrade: NormWireReader<RemoveReinforcementGrade> = normWireObject<RemoveReinforcementGrade>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
