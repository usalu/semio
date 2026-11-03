/** 🧱️ `change-element-width` wire twin: the leaf payload `ChangeElementWidth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeElementWidth {
  mutation: "changeElementWidth";
  sectionId: string;
  elementId: string;
  newWidth: number;
}

export const parseChangeElementWidth: NormWireReader<ChangeElementWidth> = normWireObject<ChangeElementWidth>({ mutation: normWireRequired(normWireLiteral("changeElementWidth")), sectionId: normWireRequired(normWireString), elementId: normWireRequired(normWireString), newWidth: normWireRequired(normWireNumber) });
