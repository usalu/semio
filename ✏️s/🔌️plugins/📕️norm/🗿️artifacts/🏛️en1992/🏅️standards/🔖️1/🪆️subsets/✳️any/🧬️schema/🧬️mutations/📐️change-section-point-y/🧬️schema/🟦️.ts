/** 📐️ `change-section-point-y` wire twin: the leaf payload `ChangeSectionPointY`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSectionPointY {
  memberId: string;
  sectionPointId: string;
  newY: number;
}

export const parseChangeSectionPointY: NormWireReader<ChangeSectionPointY> = normWireObject<ChangeSectionPointY>({ memberId: normWireRequired(normWireString), sectionPointId: normWireRequired(normWireString), newY: normWireRequired(normWireNumber) });
