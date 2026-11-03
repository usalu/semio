/** 📐️ `change-section-height` wire twin: the leaf payload `ChangeSectionHeight`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSectionHeight {
  mutation: "changeSectionHeight";
  sectionId: string;
  newHeight: number;
}

export const parseChangeSectionHeight: NormWireReader<ChangeSectionHeight> = normWireObject<ChangeSectionHeight>({ mutation: normWireRequired(normWireLiteral("changeSectionHeight")), sectionId: normWireRequired(normWireString), newHeight: normWireRequired(normWireNumber) });
