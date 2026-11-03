/** ✏️ `change-storey-stiffness-y` wire twin: the leaf payload `ChangeStoreyStiffnessY`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeStoreyStiffnessY {
  mutation: "changeStoreyStiffnessY";
  buildingId: string;
  id: string;
  newStiffnessY: number;
}

export const parseChangeStoreyStiffnessY: NormWireReader<ChangeStoreyStiffnessY> = normWireObject<ChangeStoreyStiffnessY>({ mutation: normWireRequired(normWireLiteral("changeStoreyStiffnessY")), buildingId: normWireRequired(normWireString), id: normWireRequired(normWireString), newStiffnessY: normWireRequired(normWireNumber) });
