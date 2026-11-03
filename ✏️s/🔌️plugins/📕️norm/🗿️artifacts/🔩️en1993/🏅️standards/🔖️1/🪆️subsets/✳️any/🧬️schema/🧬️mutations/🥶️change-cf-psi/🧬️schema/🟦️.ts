/** 🥶️ `change-cf-psi` wire twin: the leaf payload `ChangeCfPsi`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCfPsi {
  id: string;
  newPsi: number;
}

export const parseChangeCfPsi: NormWireReader<ChangeCfPsi> = normWireObject<ChangeCfPsi>({ id: normWireRequired(normWireString), newPsi: normWireRequired(normWireNumber) });
