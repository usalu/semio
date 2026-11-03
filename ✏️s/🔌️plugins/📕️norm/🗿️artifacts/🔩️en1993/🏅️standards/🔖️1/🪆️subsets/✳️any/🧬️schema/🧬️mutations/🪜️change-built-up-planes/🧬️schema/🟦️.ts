/** 🪜️ `change-built-up-planes` wire twin: the leaf payload `ChangeBuiltUpPlanes`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBuiltUpPlanes {
  id: string;
  newPlanes: number;
}

export const parseChangeBuiltUpPlanes: NormWireReader<ChangeBuiltUpPlanes> = normWireObject<ChangeBuiltUpPlanes>({ id: normWireRequired(normWireString), newPlanes: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
