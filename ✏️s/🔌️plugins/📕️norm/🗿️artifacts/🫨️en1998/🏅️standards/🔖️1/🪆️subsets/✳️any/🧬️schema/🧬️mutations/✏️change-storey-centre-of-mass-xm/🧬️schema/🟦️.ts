/** ✏️ `change-storey-centre-of-mass-xm` wire twin: the leaf payload `ChangeStoreyCentreOfMassXM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeStoreyCentreOfMassXM {
  mutation: "changeStoreyCentreOfMassXM";
  buildingId: string;
  id: string;
  newCentreOfMassXM: number;
}

export const parseChangeStoreyCentreOfMassXM: NormWireReader<ChangeStoreyCentreOfMassXM> = normWireObject<ChangeStoreyCentreOfMassXM>({ mutation: normWireRequired(normWireLiteral("changeStoreyCentreOfMassXM")), buildingId: normWireRequired(normWireString), id: normWireRequired(normWireString), newCentreOfMassXM: normWireRequired(normWireNumber) });
