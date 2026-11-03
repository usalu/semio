/** 🔩️ `change-joint-slip-factor-ks` wire twin: the leaf payload `ChangeJointSlipFactorKs`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointSlipFactorKs {
  id: string;
  newSlipFactorKs: number;
}

export const parseChangeJointSlipFactorKs: NormWireReader<ChangeJointSlipFactorKs> = normWireObject<ChangeJointSlipFactorKs>({ id: normWireRequired(normWireString), newSlipFactorKs: normWireRequired(normWireNumber) });
