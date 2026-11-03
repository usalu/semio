/** ✏️ `change-storey-centre-of-mass-ym` wire twin: the leaf payload `ChangeStoreyCentreOfMassYM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeStoreyCentreOfMassYM {
  mutation: "changeStoreyCentreOfMassYM";
  buildingId: string;
  id: string;
  newCentreOfMassYM: number;
}

export const parseChangeStoreyCentreOfMassYM: NormWireReader<ChangeStoreyCentreOfMassYM> = normWireObject<ChangeStoreyCentreOfMassYM>({ mutation: normWireRequired(normWireLiteral("changeStoreyCentreOfMassYM")), buildingId: normWireRequired(normWireString), id: normWireRequired(normWireString), newCentreOfMassYM: normWireRequired(normWireNumber) });
