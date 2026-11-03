/** ✏️ `change-storey-shear-resistance-n` wire twin: the leaf payload `ChangeStoreyShearResistanceN`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeStoreyShearResistanceN {
  mutation: "changeStoreyShearResistanceN";
  buildingId: string;
  id: string;
  newShearResistanceN: number;
}

export const parseChangeStoreyShearResistanceN: NormWireReader<ChangeStoreyShearResistanceN> = normWireObject<ChangeStoreyShearResistanceN>({ mutation: normWireRequired(normWireLiteral("changeStoreyShearResistanceN")), buildingId: normWireRequired(normWireString), id: normWireRequired(normWireString), newShearResistanceN: normWireRequired(normWireNumber) });
