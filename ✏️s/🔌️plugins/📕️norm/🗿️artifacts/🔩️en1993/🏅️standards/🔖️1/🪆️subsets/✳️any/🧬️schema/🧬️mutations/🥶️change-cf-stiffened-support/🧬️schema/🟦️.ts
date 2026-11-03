/** 🥶️ `change-cf-stiffened-support` wire twin: the leaf payload `ChangeCfStiffenedSupport`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCfStiffenedSupport {
  id: string;
  newStiffenedSupport: number;
}

export const parseChangeCfStiffenedSupport: NormWireReader<ChangeCfStiffenedSupport> = normWireObject<ChangeCfStiffenedSupport>({ id: normWireRequired(normWireString), newStiffenedSupport: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
