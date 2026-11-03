/** 🧨 `change-explosion-assumed-pressure` wire twin: the leaf payload `ChangeExplosionAssumedPressure`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeExplosionAssumedPressure {
  id: string;
  newAssumedPressure: number;
}

export const parseChangeExplosionAssumedPressure: NormWireReader<ChangeExplosionAssumedPressure> = normWireObject<ChangeExplosionAssumedPressure>({ id: normWireRequired(normWireString), newAssumedPressure: normWireRequired(normWireNumber) });
