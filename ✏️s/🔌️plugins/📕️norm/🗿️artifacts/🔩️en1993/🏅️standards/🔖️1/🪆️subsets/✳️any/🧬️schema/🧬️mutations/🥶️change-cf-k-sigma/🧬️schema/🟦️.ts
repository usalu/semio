/** 🥶️ `change-cf-k-sigma` wire twin: the leaf payload `ChangeCfKSigma`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCfKSigma {
  id: string;
  newKSigma: number;
}

export const parseChangeCfKSigma: NormWireReader<ChangeCfKSigma> = normWireObject<ChangeCfKSigma>({ id: normWireRequired(normWireString), newKSigma: normWireRequired(normWireNumber) });
