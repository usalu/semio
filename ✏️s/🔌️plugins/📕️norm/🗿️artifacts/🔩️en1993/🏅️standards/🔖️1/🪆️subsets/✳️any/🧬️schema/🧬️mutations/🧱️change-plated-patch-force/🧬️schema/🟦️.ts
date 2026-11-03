/** 🧱️ `change-plated-patch-force` wire twin: the leaf payload `ChangePlatedPatchForce`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePlatedPatchForce {
  id: string;
  newPatchForce: number;
}

export const parseChangePlatedPatchForce: NormWireReader<ChangePlatedPatchForce> = normWireObject<ChangePlatedPatchForce>({ id: normWireRequired(normWireString), newPatchForce: normWireRequired(normWireNumber) });
