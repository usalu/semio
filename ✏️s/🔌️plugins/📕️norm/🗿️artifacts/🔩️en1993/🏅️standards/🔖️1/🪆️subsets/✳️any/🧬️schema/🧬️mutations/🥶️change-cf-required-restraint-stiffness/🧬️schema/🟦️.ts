/** 🥶️ `change-cf-required-restraint-stiffness` wire twin: the leaf payload `ChangeCfRequiredRestraintStiffness`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCfRequiredRestraintStiffness {
  id: string;
  newRequiredRestraintStiffness: number;
}

export const parseChangeCfRequiredRestraintStiffness: NormWireReader<ChangeCfRequiredRestraintStiffness> = normWireObject<ChangeCfRequiredRestraintStiffness>({ id: normWireRequired(normWireString), newRequiredRestraintStiffness: normWireRequired(normWireNumber) });
