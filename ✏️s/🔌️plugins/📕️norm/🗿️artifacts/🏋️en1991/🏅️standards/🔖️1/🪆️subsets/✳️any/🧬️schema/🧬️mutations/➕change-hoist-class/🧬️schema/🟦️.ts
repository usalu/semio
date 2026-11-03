/** ➕ `change-hoist-class` wire twin: the leaf payload `ChangeHoistClass`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeHoistClass {
  newHoistClass: string;
}

export const parseChangeHoistClass: NormWireReader<ChangeHoistClass> = normWireObject<ChangeHoistClass>({ newHoistClass: normWireRequired(normWireString) });
