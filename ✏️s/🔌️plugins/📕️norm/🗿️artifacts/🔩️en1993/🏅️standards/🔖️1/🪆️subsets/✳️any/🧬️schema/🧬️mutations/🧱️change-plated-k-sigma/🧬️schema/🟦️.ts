/** 🧱️ `change-plated-k-sigma` wire twin: the leaf payload `ChangePlatedKSigma`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePlatedKSigma {
  id: string;
  newKSigma: number;
}

export const parseChangePlatedKSigma: NormWireReader<ChangePlatedKSigma> = normWireObject<ChangePlatedKSigma>({ id: normWireRequired(normWireString), newKSigma: normWireRequired(normWireNumber) });
