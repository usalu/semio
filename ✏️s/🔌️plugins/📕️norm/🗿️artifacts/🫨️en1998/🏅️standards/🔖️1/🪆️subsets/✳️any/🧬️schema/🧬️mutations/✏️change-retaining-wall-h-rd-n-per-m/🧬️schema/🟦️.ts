/** ✏️ `change-retaining-wall-h-rd-n-per-m` wire twin: the leaf payload `ChangeRetainingWallHRdNPerM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeRetainingWallHRdNPerM {
  mutation: "changeRetainingWallHRdNPerM";
  id: string;
  newHRdNPerM: number;
}

export const parseChangeRetainingWallHRdNPerM: NormWireReader<ChangeRetainingWallHRdNPerM> = normWireObject<ChangeRetainingWallHRdNPerM>({ mutation: normWireRequired(normWireLiteral("changeRetainingWallHRdNPerM")), id: normWireRequired(normWireString), newHRdNPerM: normWireRequired(normWireNumber) });
