/** ✏️ `change-retaining-wall-height-m` wire twin: the leaf payload `ChangeRetainingWallHeightM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeRetainingWallHeightM {
  mutation: "changeRetainingWallHeightM";
  id: string;
  newHeightM: number;
}

export const parseChangeRetainingWallHeightM: NormWireReader<ChangeRetainingWallHeightM> = normWireObject<ChangeRetainingWallHeightM>({ mutation: normWireRequired(normWireLiteral("changeRetainingWallHeightM")), id: normWireRequired(normWireString), newHeightM: normWireRequired(normWireNumber) });
