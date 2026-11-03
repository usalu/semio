/** 💧 `change-water-well-capacity` wire twin: the leaf payload `ChangeWaterWellCapacity`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeWaterWellCapacity {
  mutation: "changeWaterWellCapacity";
  id: string;
  newWellCapacity: number;
}

export const parseChangeWaterWellCapacity: NormWireReader<ChangeWaterWellCapacity> = normWireObject<ChangeWaterWellCapacity>({ mutation: normWireRequired(normWireLiteral("changeWaterWellCapacity")), id: normWireRequired(normWireString), newWellCapacity: normWireRequired(normWireNumber) });
