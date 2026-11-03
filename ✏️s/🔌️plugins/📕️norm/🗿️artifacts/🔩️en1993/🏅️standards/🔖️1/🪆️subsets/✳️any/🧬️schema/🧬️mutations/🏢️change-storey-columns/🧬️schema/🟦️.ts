/** 🏢️ `change-storey-columns` wire twin: the leaf payload `ChangeStoreyColumns`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeStoreyColumns {
  id: string;
  newColumns: number;
}

export const parseChangeStoreyColumns: NormWireReader<ChangeStoreyColumns> = normWireObject<ChangeStoreyColumns>({ id: normWireRequired(normWireString), newColumns: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
