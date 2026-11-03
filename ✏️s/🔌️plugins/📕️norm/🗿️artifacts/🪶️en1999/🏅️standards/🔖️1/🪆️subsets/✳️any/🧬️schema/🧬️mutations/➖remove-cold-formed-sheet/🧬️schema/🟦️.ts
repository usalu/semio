/** 🗂️ `remove-cold-formed-sheet` wire twin: the leaf payload `RemoveColdFormedSheet`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveColdFormedSheet {
  mutation: "removeColdFormedSheet";
  id: string;
}

export const parseRemoveColdFormedSheet: NormWireReader<RemoveColdFormedSheet> = normWireObject<RemoveColdFormedSheet>({ mutation: normWireRequired(normWireLiteral("removeColdFormedSheet")), id: normWireRequired(normWireString) });
