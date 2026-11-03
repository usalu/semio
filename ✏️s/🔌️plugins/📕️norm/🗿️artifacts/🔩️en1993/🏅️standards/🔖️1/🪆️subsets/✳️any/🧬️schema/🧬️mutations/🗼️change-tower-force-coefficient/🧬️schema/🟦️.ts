/** 🗼️ `change-tower-force-coefficient` wire twin: the leaf payload `ChangeTowerForceCoefficient`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTowerForceCoefficient {
  id: string;
  newForceCoefficient: number;
}

export const parseChangeTowerForceCoefficient: NormWireReader<ChangeTowerForceCoefficient> = normWireObject<ChangeTowerForceCoefficient>({ id: normWireRequired(normWireString), newForceCoefficient: normWireRequired(normWireNumber) });
