/** 📐️ `change-section-web-thickness` wire twin: the leaf payload `ChangeSectionWebThickness`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSectionWebThickness {
  mutation: "changeSectionWebThickness";
  sectionId: string;
  newWebThickness: number;
}

export const parseChangeSectionWebThickness: NormWireReader<ChangeSectionWebThickness> = normWireObject<ChangeSectionWebThickness>({ mutation: normWireRequired(normWireLiteral("changeSectionWebThickness")), sectionId: normWireRequired(normWireString), newWebThickness: normWireRequired(normWireNumber) });
