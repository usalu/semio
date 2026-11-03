/** 💥 `change-crane-class` wire twin: the leaf payload `ChangeCraneClass`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCraneClass {
  newCraneClass: string;
}

export const parseChangeCraneClass: NormWireReader<ChangeCraneClass> = normWireObject<ChangeCraneClass>({ newCraneClass: normWireRequired(normWireString) });
