/** 🔩️ `change-row-bolt-class` wire twin: the leaf payload `ChangeRowBoltClass`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeRowBoltClass {
  id: string;
  newBoltClass: string;
}

export const parseChangeRowBoltClass: NormWireReader<ChangeRowBoltClass> = normWireObject<ChangeRowBoltClass>({ id: normWireRequired(normWireString), newBoltClass: normWireRequired(normWireString) });
