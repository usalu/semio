/** 🥶️ `change-cf-purlin-load` wire twin: the leaf payload `ChangeCfPurlinLoad`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCfPurlinLoad {
  id: string;
  newPurlinLoad: number;
}

export const parseChangeCfPurlinLoad: NormWireReader<ChangeCfPurlinLoad> = normWireObject<ChangeCfPurlinLoad>({ id: normWireRequired(normWireString), newPurlinLoad: normWireRequired(normWireNumber) });
