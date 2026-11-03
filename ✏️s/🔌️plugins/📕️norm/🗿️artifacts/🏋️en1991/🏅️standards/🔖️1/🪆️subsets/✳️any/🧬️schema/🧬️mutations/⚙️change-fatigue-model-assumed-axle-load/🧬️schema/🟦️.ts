/** ⚙ `change-fatigue-model-assumed-axle-load` wire twin: the leaf payload `ChangeFatigueModelAssumedAxleLoad`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFatigueModelAssumedAxleLoad {
  id: string;
  newAssumedAxleLoad: number;
}

export const parseChangeFatigueModelAssumedAxleLoad: NormWireReader<ChangeFatigueModelAssumedAxleLoad> = normWireObject<ChangeFatigueModelAssumedAxleLoad>({ id: normWireRequired(normWireString), newAssumedAxleLoad: normWireRequired(normWireNumber) });
