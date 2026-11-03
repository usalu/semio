/** ✏️ `change-member-effective-stiffness-ratio` wire twin: the leaf payload `ChangeMemberEffectiveStiffnessRatio`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberEffectiveStiffnessRatio {
  mutation: "changeMemberEffectiveStiffnessRatio";
  buildingId: string;
  id: string;
  newEffectiveStiffnessRatio: number;
}

export const parseChangeMemberEffectiveStiffnessRatio: NormWireReader<ChangeMemberEffectiveStiffnessRatio> = normWireObject<ChangeMemberEffectiveStiffnessRatio>({ mutation: normWireRequired(normWireLiteral("changeMemberEffectiveStiffnessRatio")), buildingId: normWireRequired(normWireString), id: normWireRequired(normWireString), newEffectiveStiffnessRatio: normWireRequired(normWireNumber) });
