/** ✏️ `change-storey-centre-of-stiffness-xm` wire twin: the leaf payload `ChangeStoreyCentreOfStiffnessXM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeStoreyCentreOfStiffnessXM {
  mutation: "changeStoreyCentreOfStiffnessXM";
  buildingId: string;
  id: string;
  newCentreOfStiffnessXM: number;
}

export const parseChangeStoreyCentreOfStiffnessXM: NormWireReader<ChangeStoreyCentreOfStiffnessXM> = normWireObject<ChangeStoreyCentreOfStiffnessXM>({ mutation: normWireRequired(normWireLiteral("changeStoreyCentreOfStiffnessXM")), buildingId: normWireRequired(normWireString), id: normWireRequired(normWireString), newCentreOfStiffnessXM: normWireRequired(normWireNumber) });
