/** ✏️ `change-retaining-wall-permeability-ms` wire twin: the leaf payload `ChangeRetainingWallPermeabilityMS`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireNumber, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeRetainingWallPermeabilityMS {
  mutation: "changeRetainingWallPermeabilityMS";
  index: number;
  newPermeabilityMS: number;
}

export const parseChangeRetainingWallPermeabilityMS: NormWireReader<ChangeRetainingWallPermeabilityMS> = normWireObject<ChangeRetainingWallPermeabilityMS>({ mutation: normWireRequired(normWireLiteral("changeRetainingWallPermeabilityMS")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newPermeabilityMS: normWireRequired(normWireNumber) });
