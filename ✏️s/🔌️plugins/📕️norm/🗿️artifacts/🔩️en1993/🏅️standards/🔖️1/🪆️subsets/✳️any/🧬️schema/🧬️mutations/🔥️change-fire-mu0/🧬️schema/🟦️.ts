/** 🔥️ `change-fire-mu0` wire twin: the leaf payload `ChangeFireMu0`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFireMu0 {
  id: string;
  newMu0: number;
}

export const parseChangeFireMu0: NormWireReader<ChangeFireMu0> = normWireObject<ChangeFireMu0>({ id: normWireRequired(normWireString), newMu0: normWireRequired(normWireNumber) });
