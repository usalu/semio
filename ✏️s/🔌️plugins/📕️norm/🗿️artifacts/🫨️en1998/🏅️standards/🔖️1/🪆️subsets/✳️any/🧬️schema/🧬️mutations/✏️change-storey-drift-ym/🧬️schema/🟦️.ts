/** ✏️ `change-storey-drift-ym` wire twin: the leaf payload `ChangeStoreyDriftYM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeStoreyDriftYM {
  mutation: "changeStoreyDriftYM";
  buildingId: string;
  id: string;
  newDriftYM: number;
}

export const parseChangeStoreyDriftYM: NormWireReader<ChangeStoreyDriftYM> = normWireObject<ChangeStoreyDriftYM>({ mutation: normWireRequired(normWireLiteral("changeStoreyDriftYM")), buildingId: normWireRequired(normWireString), id: normWireRequired(normWireString), newDriftYM: normWireRequired(normWireNumber) });
