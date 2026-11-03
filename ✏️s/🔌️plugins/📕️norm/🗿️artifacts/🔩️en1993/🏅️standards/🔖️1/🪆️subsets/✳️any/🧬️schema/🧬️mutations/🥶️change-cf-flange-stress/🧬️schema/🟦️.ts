/** 🥶️ `change-cf-flange-stress` wire twin: the leaf payload `ChangeCfFlangeStress`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCfFlangeStress {
  id: string;
  newFlangeStress: number;
}

export const parseChangeCfFlangeStress: NormWireReader<ChangeCfFlangeStress> = normWireObject<ChangeCfFlangeStress>({ id: normWireRequired(normWireString), newFlangeStress: normWireRequired(normWireNumber) });
