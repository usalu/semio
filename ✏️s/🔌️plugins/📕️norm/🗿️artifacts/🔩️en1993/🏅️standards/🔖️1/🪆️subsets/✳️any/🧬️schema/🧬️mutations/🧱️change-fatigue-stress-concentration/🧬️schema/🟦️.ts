/** 🧱️ `change-fatigue-stress-concentration` wire twin: the leaf payload `ChangeFatigueStressConcentration`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFatigueStressConcentration {
  id: string;
  newConcentration: number;
}

export const parseChangeFatigueStressConcentration: NormWireReader<ChangeFatigueStressConcentration> = normWireObject<ChangeFatigueStressConcentration>({ id: normWireRequired(normWireString), newConcentration: normWireRequired(normWireNumber) });
