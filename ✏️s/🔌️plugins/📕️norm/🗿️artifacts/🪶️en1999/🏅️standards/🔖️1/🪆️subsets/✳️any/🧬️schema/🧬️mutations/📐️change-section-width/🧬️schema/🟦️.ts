/** 📐️ `change-section-width` wire twin: the leaf payload `ChangeSectionWidth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSectionWidth {
  mutation: "changeSectionWidth";
  sectionId: string;
  newWidth: number;
}

export const parseChangeSectionWidth: NormWireReader<ChangeSectionWidth> = normWireObject<ChangeSectionWidth>({ mutation: normWireRequired(normWireLiteral("changeSectionWidth")), sectionId: normWireRequired(normWireString), newWidth: normWireRequired(normWireNumber) });
