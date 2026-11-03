/** ✏️ `change-storey-centre-of-stiffness-ym` wire twin: the leaf payload `ChangeStoreyCentreOfStiffnessYM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeStoreyCentreOfStiffnessYM {
  mutation: "changeStoreyCentreOfStiffnessYM";
  buildingId: string;
  id: string;
  newCentreOfStiffnessYM: number;
}

export const parseChangeStoreyCentreOfStiffnessYM: NormWireReader<ChangeStoreyCentreOfStiffnessYM> = normWireObject<ChangeStoreyCentreOfStiffnessYM>({ mutation: normWireRequired(normWireLiteral("changeStoreyCentreOfStiffnessYM")), buildingId: normWireRequired(normWireString), id: normWireRequired(normWireString), newCentreOfStiffnessYM: normWireRequired(normWireNumber) });
