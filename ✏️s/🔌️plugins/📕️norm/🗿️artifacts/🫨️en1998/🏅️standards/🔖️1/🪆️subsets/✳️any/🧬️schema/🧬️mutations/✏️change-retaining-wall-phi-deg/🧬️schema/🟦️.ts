/** ✏️ `change-retaining-wall-phi-deg` wire twin: the leaf payload `ChangeRetainingWallPhiDeg`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeRetainingWallPhiDeg {
  mutation: "changeRetainingWallPhiDeg";
  id: string;
  newPhiDeg: number;
}

export const parseChangeRetainingWallPhiDeg: NormWireReader<ChangeRetainingWallPhiDeg> = normWireObject<ChangeRetainingWallPhiDeg>({ mutation: normWireRequired(normWireLiteral("changeRetainingWallPhiDeg")), id: normWireRequired(normWireString), newPhiDeg: normWireRequired(normWireNumber) });
