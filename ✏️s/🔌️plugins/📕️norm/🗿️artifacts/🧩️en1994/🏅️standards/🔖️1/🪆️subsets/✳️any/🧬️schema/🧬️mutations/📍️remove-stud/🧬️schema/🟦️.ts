/** 📍️ `remove-stud` wire twin: the leaf payload `RemoveStud`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveStud {
  id: string;
}

export const parseRemoveStud: NormWireReader<RemoveStud> = normWireObject<RemoveStud>({ id: normWireRequired(normWireString) });
