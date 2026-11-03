/** 🥶️ `change-cf-required-restraint-stiff` wire twin: the leaf payload `ChangeCfRequiredRestraintStiff`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCfRequiredRestraintStiff {
  id: string;
  newRequiredRestraintStiffness: number;
}

export const parseChangeCfRequiredRestraintStiff: NormWireReader<ChangeCfRequiredRestraintStiff> = normWireObject<ChangeCfRequiredRestraintStiff>({ id: normWireRequired(normWireString), newRequiredRestraintStiffness: normWireRequired(normWireNumber) });
