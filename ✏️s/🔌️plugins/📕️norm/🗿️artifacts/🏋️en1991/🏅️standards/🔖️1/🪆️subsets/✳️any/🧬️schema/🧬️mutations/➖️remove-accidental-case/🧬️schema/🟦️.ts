/** ➖️ `remove-accidental-case` wire twin: the leaf payload `RemoveAccidentalCase`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveAccidentalCase {
  id: string;
}

export const parseRemoveAccidentalCase: NormWireReader<RemoveAccidentalCase> = normWireObject<RemoveAccidentalCase>({ id: normWireRequired(normWireString) });
