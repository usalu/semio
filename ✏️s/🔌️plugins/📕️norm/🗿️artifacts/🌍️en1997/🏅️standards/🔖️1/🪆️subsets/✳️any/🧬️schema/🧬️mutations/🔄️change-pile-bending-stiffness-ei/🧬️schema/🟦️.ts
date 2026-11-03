/** 🔄️ `change-pile-bending-stiffness-ei` wire twin: the leaf payload `ChangePileBendingStiffnessEi`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePileBendingStiffnessEi {
  mutation: "changePileBendingStiffnessEi";
  id: string;
  newBendingStiffnessEi: number;
}

export const parseChangePileBendingStiffnessEi: NormWireReader<ChangePileBendingStiffnessEi> = normWireObject<ChangePileBendingStiffnessEi>({ mutation: normWireRequired(normWireLiteral("changePileBendingStiffnessEi")), id: normWireRequired(normWireString), newBendingStiffnessEi: normWireRequired(normWireNumber) });
