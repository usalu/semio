/** ✏️ `change-retaining-wall-r` wire twin: the leaf payload `ChangeRetainingWallR`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeRetainingWallR {
  mutation: "changeRetainingWallR";
  id: string;
  newR: number;
}

export const parseChangeRetainingWallR: NormWireReader<ChangeRetainingWallR> = normWireObject<ChangeRetainingWallR>({ mutation: normWireRequired(normWireLiteral("changeRetainingWallR")), id: normWireRequired(normWireString), newR: normWireRequired(normWireNumber) });
