/** 🔩️ `change-row-alpha-factor` wire twin: the leaf payload `ChangeRowAlphaFactor`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeRowAlphaFactor {
  id: string;
  newAlphaFactor: number;
}

export const parseChangeRowAlphaFactor: NormWireReader<ChangeRowAlphaFactor> = normWireObject<ChangeRowAlphaFactor>({ id: normWireRequired(normWireString), newAlphaFactor: normWireRequired(normWireNumber) });
