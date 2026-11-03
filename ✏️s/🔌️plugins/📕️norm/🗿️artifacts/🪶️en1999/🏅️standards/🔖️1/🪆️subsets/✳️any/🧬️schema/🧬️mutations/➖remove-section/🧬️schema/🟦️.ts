/** 🗂️ `remove-section` wire twin: the leaf payload `RemoveSection`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveSection {
  mutation: "removeSection";
  id: string;
}

export const parseRemoveSection: NormWireReader<RemoveSection> = normWireObject<RemoveSection>({ mutation: normWireRequired(normWireLiteral("removeSection")), id: normWireRequired(normWireString) });
