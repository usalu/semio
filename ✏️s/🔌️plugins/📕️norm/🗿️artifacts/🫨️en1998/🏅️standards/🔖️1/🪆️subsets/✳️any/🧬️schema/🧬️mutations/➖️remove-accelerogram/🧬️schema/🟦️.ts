/** ➖️ `remove-accelerogram` wire twin: the leaf payload `RemoveAccelerogram`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveAccelerogram {
  mutation: "removeAccelerogram";
  id: string;
}

export const parseRemoveAccelerogram: NormWireReader<RemoveAccelerogram> = normWireObject<RemoveAccelerogram>({ mutation: normWireRequired(normWireLiteral("removeAccelerogram")), id: normWireRequired(normWireString) });
