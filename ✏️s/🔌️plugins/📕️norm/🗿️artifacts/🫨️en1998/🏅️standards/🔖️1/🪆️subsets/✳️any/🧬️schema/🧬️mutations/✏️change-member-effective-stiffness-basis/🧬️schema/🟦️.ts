/** ✏️ `change-member-effective-stiffness-basis` wire twin: the leaf payload `ChangeMemberEffectiveStiffnessBasis`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberEffectiveStiffnessBasis {
  mutation: "changeMemberEffectiveStiffnessBasis";
  buildingId: string;
  id: string;
  newEffectiveStiffnessBasis: "simplified" | "declared";
}

export const parseChangeMemberEffectiveStiffnessBasis: NormWireReader<ChangeMemberEffectiveStiffnessBasis> = normWireObject<ChangeMemberEffectiveStiffnessBasis>({ mutation: normWireRequired(normWireLiteral("changeMemberEffectiveStiffnessBasis")), buildingId: normWireRequired(normWireString), id: normWireRequired(normWireString), newEffectiveStiffnessBasis: normWireRequired(normWireLiteral("simplified", "declared")) });
