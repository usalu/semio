/** 📐️ `change-section-outer-diameter` wire twin: the leaf payload `ChangeSectionOuterDiameter`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSectionOuterDiameter {
  mutation: "changeSectionOuterDiameter";
  sectionId: string;
  newOuterDiameter: number;
}

export const parseChangeSectionOuterDiameter: NormWireReader<ChangeSectionOuterDiameter> = normWireObject<ChangeSectionOuterDiameter>({ mutation: normWireRequired(normWireLiteral("changeSectionOuterDiameter")), sectionId: normWireRequired(normWireString), newOuterDiameter: normWireRequired(normWireNumber) });
