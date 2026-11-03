/** 🗂️ `insert-outline-vertex` wire twin: the leaf payload `InsertOutlineVertex`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type SectionVertex, parseSectionVertex } from "../../../📸️snapshot/🟦️.ts";

export interface InsertOutlineVertex {
  mutation: "insertOutlineVertex";
  index: number;
  outlineVertex: SectionVertex;
}

export const parseInsertOutlineVertex: NormWireReader<InsertOutlineVertex> = normWireObject<InsertOutlineVertex>({ mutation: normWireRequired(normWireLiteral("insertOutlineVertex")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), outlineVertex: normWireRequired(parseSectionVertex) });
