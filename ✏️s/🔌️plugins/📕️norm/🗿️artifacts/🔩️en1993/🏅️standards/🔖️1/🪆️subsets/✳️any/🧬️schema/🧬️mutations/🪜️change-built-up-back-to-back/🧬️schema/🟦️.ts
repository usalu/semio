/** 🪜️ `change-built-up-back-to-back` wire twin: the leaf payload `ChangeBuiltUpBackToBack`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBuiltUpBackToBack {
  id: string;
  newBackToBack: number;
}

export const parseChangeBuiltUpBackToBack: NormWireReader<ChangeBuiltUpBackToBack> = normWireObject<ChangeBuiltUpBackToBack>({ id: normWireRequired(normWireString), newBackToBack: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
