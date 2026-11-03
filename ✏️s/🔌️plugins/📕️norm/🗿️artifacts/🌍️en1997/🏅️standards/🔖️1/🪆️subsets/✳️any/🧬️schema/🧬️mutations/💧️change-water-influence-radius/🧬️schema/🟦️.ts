/** 💧 `change-water-influence-radius` wire twin: the leaf payload `ChangeWaterInfluenceRadius`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeWaterInfluenceRadius {
  mutation: "changeWaterInfluenceRadius";
  id: string;
  newInfluenceRadius: number;
}

export const parseChangeWaterInfluenceRadius: NormWireReader<ChangeWaterInfluenceRadius> = normWireObject<ChangeWaterInfluenceRadius>({ mutation: normWireRequired(normWireLiteral("changeWaterInfluenceRadius")), id: normWireRequired(normWireString), newInfluenceRadius: normWireRequired(normWireNumber) });
