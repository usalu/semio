/** 🔗 `change-tie-run-provided-force` wire twin: the leaf payload `ChangeTieRunProvidedForce`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTieRunProvidedForce {
  id: string;
  newProvidedForce: number;
}

export const parseChangeTieRunProvidedForce: NormWireReader<ChangeTieRunProvidedForce> = normWireObject<ChangeTieRunProvidedForce>({ id: normWireRequired(normWireString), newProvidedForce: normWireRequired(normWireNumber) });
