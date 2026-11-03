/** 🗂️ `insert-section-element` wire twin: the leaf payload `InsertSectionElement`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type PlateElement, parsePlateElement } from "../../../📸️snapshot/🟦️.ts";

export interface InsertSectionElement {
  mutation: "insertSectionElement";
  sectionId: string;
  index: number;
  element: PlateElement;
}

export const parseInsertSectionElement: NormWireReader<InsertSectionElement> = normWireObject<InsertSectionElement>({ mutation: normWireRequired(normWireLiteral("insertSectionElement")), sectionId: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), element: normWireRequired(parsePlateElement) });
