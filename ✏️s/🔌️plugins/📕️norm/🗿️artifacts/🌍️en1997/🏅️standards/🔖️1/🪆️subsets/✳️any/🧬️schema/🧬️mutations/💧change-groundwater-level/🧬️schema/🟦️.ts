/** 💧 `change-groundwater-level` wire twin: the leaf payload `ChangeGroundwaterLevel`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeGroundwaterLevel {
  mutation: "changeGroundwaterLevel";
  newGroundwaterLevel: number;
}

export const parseChangeGroundwaterLevel: NormWireReader<ChangeGroundwaterLevel> = normWireObject<ChangeGroundwaterLevel>({ mutation: normWireRequired(normWireLiteral("changeGroundwaterLevel")), newGroundwaterLevel: normWireRequired(normWireNumber) });
