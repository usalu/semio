/** 💧 `change-groundwater-level` wire twin: the leaf payload `ChangeGroundwaterLevel`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeGroundwaterLevel {
  mutation: "changeGroundwaterLevel";
  newGroundwaterLevel: number;
}

export const parseChangeGroundwaterLevel: NormWireReader<ChangeGroundwaterLevel> = normWireObject<ChangeGroundwaterLevel>({ mutation: normWireRequired(normWireLiteral("changeGroundwaterLevel")), newGroundwaterLevel: normWireRequired(normWireNumber) });
