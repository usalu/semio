/** 🔩️ `change-joint-alpha-factor` wire twin: the leaf payload `ChangeJointAlphaFactor`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointAlphaFactor {
  id: string;
  newAlphaFactor: number;
}

export const parseChangeJointAlphaFactor: NormWireReader<ChangeJointAlphaFactor> = normWireObject<ChangeJointAlphaFactor>({ id: normWireRequired(normWireString), newAlphaFactor: normWireRequired(normWireNumber) });
