/** 🗂️ `remove-section-element` wire twin: the leaf payload `RemoveSectionElement`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveSectionElement {
  mutation: "removeSectionElement";
  sectionId: string;
  id: string;
}

export const parseRemoveSectionElement: NormWireReader<RemoveSectionElement> = normWireObject<RemoveSectionElement>({ mutation: normWireRequired(normWireLiteral("removeSectionElement")), sectionId: normWireRequired(normWireString), id: normWireRequired(normWireString) });
