/** ✏️ `change-member-steel-section-class` wire twin: the leaf payload `ChangeMemberSteelSectionClass`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberSteelSectionClass {
  mutation: "changeMemberSteelSectionClass";
  buildingId: string;
  id: string;
  newSteelSectionClass: number;
}

export const parseChangeMemberSteelSectionClass: NormWireReader<ChangeMemberSteelSectionClass> = normWireObject<ChangeMemberSteelSectionClass>({ mutation: normWireRequired(normWireLiteral("changeMemberSteelSectionClass")), buildingId: normWireRequired(normWireString), id: normWireRequired(normWireString), newSteelSectionClass: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":255})) });
