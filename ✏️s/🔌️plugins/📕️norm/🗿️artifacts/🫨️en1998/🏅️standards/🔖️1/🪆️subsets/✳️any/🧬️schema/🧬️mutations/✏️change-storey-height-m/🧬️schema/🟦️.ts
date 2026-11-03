/** ✏️ `change-storey-height-m` wire twin: the leaf payload `ChangeStoreyHeightM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeStoreyHeightM {
  mutation: "changeStoreyHeightM";
  buildingId: string;
  id: string;
  newHeightM: number;
}

export const parseChangeStoreyHeightM: NormWireReader<ChangeStoreyHeightM> = normWireObject<ChangeStoreyHeightM>({ mutation: normWireRequired(normWireLiteral("changeStoreyHeightM")), buildingId: normWireRequired(normWireString), id: normWireRequired(normWireString), newHeightM: normWireRequired(normWireNumber) });
