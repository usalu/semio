/** 📐️ `change-storey-stiffness-x` wire twin: the leaf payload `ChangeStoreyStiffnessX`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeStoreyStiffnessX {
  mutation: "changeStoreyStiffnessX";
  buildingId: string;
  id: string;
  newStiffnessX: number;
}

export const parseChangeStoreyStiffnessX: NormWireReader<ChangeStoreyStiffnessX> = normWireObject<ChangeStoreyStiffnessX>({ mutation: normWireRequired(normWireLiteral("changeStoreyStiffnessX")), buildingId: normWireRequired(normWireString), id: normWireRequired(normWireString), newStiffnessX: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
