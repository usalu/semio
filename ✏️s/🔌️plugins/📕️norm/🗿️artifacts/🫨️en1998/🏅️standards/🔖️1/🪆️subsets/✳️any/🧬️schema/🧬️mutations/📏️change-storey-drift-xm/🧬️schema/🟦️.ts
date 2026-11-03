/** 📏️ `change-storey-drift-xm` wire twin: the leaf payload `ChangeStoreyDriftXM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeStoreyDriftXM {
  mutation: "changeStoreyDriftXM";
  buildingId: string;
  id: string;
  newDriftXM: number;
}

export const parseChangeStoreyDriftXM: NormWireReader<ChangeStoreyDriftXM> = normWireObject<ChangeStoreyDriftXM>({ mutation: normWireRequired(normWireLiteral("changeStoreyDriftXM")), buildingId: normWireRequired(normWireString), id: normWireRequired(normWireString), newDriftXM: normWireRequired(normWireNumber) });
