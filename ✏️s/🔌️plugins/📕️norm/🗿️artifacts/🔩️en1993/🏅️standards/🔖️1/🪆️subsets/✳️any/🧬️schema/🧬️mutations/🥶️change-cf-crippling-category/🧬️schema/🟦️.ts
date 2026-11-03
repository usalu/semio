/** 🥶️ `change-cf-crippling-category` wire twin: the leaf payload `ChangeCfCripplingCategory`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCfCripplingCategory {
  id: string;
  newCripplingCategory: number;
}

export const parseChangeCfCripplingCategory: NormWireReader<ChangeCfCripplingCategory> = normWireObject<ChangeCfCripplingCategory>({ id: normWireRequired(normWireString), newCripplingCategory: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
