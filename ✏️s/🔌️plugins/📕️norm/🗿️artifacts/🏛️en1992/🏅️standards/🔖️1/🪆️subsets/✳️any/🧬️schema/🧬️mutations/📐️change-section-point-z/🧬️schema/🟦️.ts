/** 📐️ `change-section-point-z` wire twin: the leaf payload `ChangeSectionPointZ`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSectionPointZ {
  memberId: string;
  sectionPointId: string;
  newZ: number;
}

export const parseChangeSectionPointZ: NormWireReader<ChangeSectionPointZ> = normWireObject<ChangeSectionPointZ>({ memberId: normWireRequired(normWireString), sectionPointId: normWireRequired(normWireString), newZ: normWireRequired(normWireNumber) });
