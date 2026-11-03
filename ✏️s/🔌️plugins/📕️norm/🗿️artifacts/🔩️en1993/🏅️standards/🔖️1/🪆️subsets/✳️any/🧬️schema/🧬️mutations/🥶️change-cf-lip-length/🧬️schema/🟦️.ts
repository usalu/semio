/** 🥶️ `change-cf-lip-length` wire twin: the leaf payload `ChangeCfLipLength`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCfLipLength {
  id: string;
  newLipLength: number;
}

export const parseChangeCfLipLength: NormWireReader<ChangeCfLipLength> = normWireObject<ChangeCfLipLength>({ id: normWireRequired(normWireString), newLipLength: normWireRequired(normWireNumber) });
