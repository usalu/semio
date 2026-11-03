/** 💧 `change-water-well-water-depth` wire twin: the leaf payload `ChangeWaterWellWaterDepth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeWaterWellWaterDepth {
  mutation: "changeWaterWellWaterDepth";
  id: string;
  newWellWaterDepth: number;
}

export const parseChangeWaterWellWaterDepth: NormWireReader<ChangeWaterWellWaterDepth> = normWireObject<ChangeWaterWellWaterDepth>({ mutation: normWireRequired(normWireLiteral("changeWaterWellWaterDepth")), id: normWireRequired(normWireString), newWellWaterDepth: normWireRequired(normWireNumber) });
