/** 🏗 `change-fatigue-model-lane` wire twin: the leaf payload `ChangeFatigueModelLane`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFatigueModelLane {
  id: string;
  newLane: number;
}

export const parseChangeFatigueModelLane: NormWireReader<ChangeFatigueModelLane> = normWireObject<ChangeFatigueModelLane>({ id: normWireRequired(normWireString), newLane: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":255})) });
