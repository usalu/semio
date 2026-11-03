/** 📐️ `change-section-flange-thickness` wire twin: the leaf payload `ChangeSectionFlangeThickness`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSectionFlangeThickness {
  mutation: "changeSectionFlangeThickness";
  sectionId: string;
  newFlangeThickness: number;
}

export const parseChangeSectionFlangeThickness: NormWireReader<ChangeSectionFlangeThickness> = normWireObject<ChangeSectionFlangeThickness>({ mutation: normWireRequired(normWireLiteral("changeSectionFlangeThickness")), sectionId: normWireRequired(normWireString), newFlangeThickness: normWireRequired(normWireNumber) });
