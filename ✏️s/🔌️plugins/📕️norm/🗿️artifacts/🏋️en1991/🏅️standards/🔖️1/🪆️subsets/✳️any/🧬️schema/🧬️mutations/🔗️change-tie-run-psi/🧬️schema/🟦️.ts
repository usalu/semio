/** 🔗 `change-tie-run-psi` wire twin: the leaf payload `ChangeTieRunPsi`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTieRunPsi {
  id: string;
  newPsi: number;
}

export const parseChangeTieRunPsi: NormWireReader<ChangeTieRunPsi> = normWireObject<ChangeTieRunPsi>({ id: normWireRequired(normWireString), newPsi: normWireRequired(normWireNumber) });
