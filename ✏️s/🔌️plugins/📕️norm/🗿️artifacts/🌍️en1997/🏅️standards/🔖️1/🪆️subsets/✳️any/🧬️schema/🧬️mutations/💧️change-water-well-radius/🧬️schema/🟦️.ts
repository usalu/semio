/** 💧 `change-water-well-radius` wire twin: the leaf payload `ChangeWaterWellRadius`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeWaterWellRadius {
  mutation: "changeWaterWellRadius";
  id: string;
  newWellRadius: number;
}

export const parseChangeWaterWellRadius: NormWireReader<ChangeWaterWellRadius> = normWireObject<ChangeWaterWellRadius>({ mutation: normWireRequired(normWireLiteral("changeWaterWellRadius")), id: normWireRequired(normWireString), newWellRadius: normWireRequired(normWireNumber) });
